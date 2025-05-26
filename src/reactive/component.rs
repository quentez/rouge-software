use std::fmt::{Debug, Error, Formatter};
use std::pin::Pin;
use std::sync::RwLock;
use std::task::{Context, Poll};

use adw::glib::clone::Downgrade;
use adw::glib::object::IsA;
use adw::glib::{Object, WeakRef};
use colored::Colorize;
use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::stream::{select, Stream};
use futures::StreamExt;
use log::{debug, trace};

use crate::reactive::scope::Scope;
use crate::reactive::vnode::VNode;

use super::callback::Callback;
use super::scope::AnyScope;
use super::vstate::VState;

pub enum UpdateAction {
  None,
  Render,
}

pub struct ViewContext<C: Component> {
  scope: Scope<C>,
}

impl<C: Component> ViewContext<C> {
  pub fn new(scope: Scope<C>) -> ViewContext<C> {
    ViewContext { scope }
  }
}

impl<C: 'static + Component> ViewContext<C> {
  pub fn d<R, MB: 'static + Fn(R) -> C::Message>(&self, message_builder: MB) -> Callback<R> {
    let scope_clone = self.scope.clone();
    Callback::from(move |o| {
      let message = message_builder(o);
      scope_clone.send_message(message);
    })
  }
}

pub trait Component: Default + Unpin + Clone {
  type Message: Clone + Send + Debug + Unpin;
  type Props: Clone + Default;

  fn update(&mut self, _message: Self::Message) -> UpdateAction {
    UpdateAction::None
  }

  fn create(_props: Self::Props) -> Self {
    Default::default()
  }

  fn change(&mut self, _props: Self::Props) -> UpdateAction {
    unimplemented!("add a Component::change() implementation");
  }

  fn mounted(&self) {}
  fn unmounted(&self) {}

  fn view(&self, context: &ViewContext<Self>) -> VNode<Self>;
}

pub enum ComponentMessage<C: Component> {
  Update(C::Message),
  Props(C::Props),
  Mounted,
  Unmounted,
}

impl<C: Component> Debug for ComponentMessage<C> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    match self {
      ComponentMessage::Update(msg) => write!(
        f,
        "{}",
        format!(
          "ComponentMessage::Update({})",
          format!("{:?}", msg).bright_white().bold()
        )
        .green()
      ),
      ComponentMessage::Props(_) => write!(f, "{}", "ComponentMessage::Props(...)".green()),
      ComponentMessage::Mounted => write!(f, "{}", "ComponentMessage::Mounted".green()),
      ComponentMessage::Unmounted => write!(f, "{}", "ComponentMessage::Unmounted".green()),
    }
  }
}

pub struct ComponentTask<C, P>
where
  C: Component,
  P: Component,
{
  scope: Scope<C>,
  parent_scope: Option<Scope<P>>,
  state: C,
  ui_state: Option<VState<C>>,
  channel: Pin<Box<dyn Stream<Item = ComponentMessage<C>>>>,
}

impl<C, P> ComponentTask<C, P>
where
  C: 'static + Component,
  P: 'static + Component,
{
  pub fn new(
    props: C::Props,
    parent: Option<&Object>,
    parent_scope: Option<&Scope<P>>,
  ) -> (UnboundedSender<ComponentMessage<C>>, Self) {
    PartialComponentTask::new(props, parent, parent_scope).finalise()
  }

  pub fn process(&mut self, ctx: &mut Context<'_>) -> Poll<()> {
    let mut render = false;
    loop {
      let next = Stream::poll_next(self.channel.as_mut(), ctx);
      trace!(
        "{} {}",
        self.scope.name().bright_black(),
        format!("{:?}", next).bright_black().bold()
      );
      match next {
        Poll::Ready(Some(msg)) => match msg {
          ComponentMessage::Update(msg) => {
            let result = self.state.update(msg);
            match result {
              // UpdateAction::Defer(job) => {
              //   self.run_job(job);
              // }
              UpdateAction::Render => {
                render = true;
              }
              UpdateAction::None => {}
            }
          }
          ComponentMessage::Props(props) => match self.state.change(props) {
            // UpdateAction::Defer(job) => {
            //   self.run_job(job);
            // }
            UpdateAction::Render => {
              render = true;
            }
            UpdateAction::None => {}
          },
          ComponentMessage::Mounted => {
            debug!(
              "{} {}",
              "Component mounted:".bright_blue(),
              self.scope.name().magenta().bold()
            );
            self.state.mounted();
          }
          ComponentMessage::Unmounted => {
            if let Some(state) = self.ui_state.take() {
              state.unmount();
            }
            self.state.unmounted();
            debug!(
              "{} {}",
              "Component unmounted:".bright_red(),
              self.scope.name().magenta().bold()
            );
            return Poll::Ready(());
          }
        },
        Poll::Pending if render => {
          if let Some(ref mut ui_state) = self.ui_state {
            // we patch
            let context = ViewContext::new(self.scope.clone());
            let new_view = self.state.view(&context);
            self.scope.mute();
            if !ui_state.patch(&new_view, None, &self.scope) {
              unimplemented!(
                "{}: don't know how to propagate failed patch",
                self.scope.name()
              );
            }
            self.scope.unmute();
            return Poll::Pending;
          } else {
            debug!(
              "{} {}",
              self.scope.name().magenta().bold(),
              "rendering in the absence of a UI state; exiting".bright_red()
            );
            return Poll::Ready(());
          }
        }
        Poll::Ready(None) => {
          debug!(
            "{} {}",
            self.scope.name().magenta().bold(),
            "terminating because all channel handles dropped".bright_red()
          );
          return Poll::Ready(());
        }
        Poll::Pending => return Poll::Pending,
      }
    }
  }

  pub fn object(&self) -> Option<Object> {
    self.ui_state.as_ref().map(|state| state.object().clone())
  }
}

pub struct PartialComponentTask<C, P>
where
  C: Component,
  P: Component,
{
  task: ComponentTask<C, P>,
  // view: VNode<C>,
  sender: UnboundedSender<ComponentMessage<C>>,
}

impl<C, P> PartialComponentTask<C, P>
where
  C: 'static + Component,
  P: 'static + Component,
{
  /// Start building a `ComponentTask` by initialising the task and the root
  /// object but not the children.
  ///
  /// This is generally only useful when you're constructing an `Application`,
  /// where windows should not be added to it until it's been activated, but
  /// you need to have the `Application` object in order to activate it.
  pub fn new(props: C::Props, parent: Option<&Object>, parent_scope: Option<&Scope<P>>) -> Self {
    let (sys_send, sys_recv) = unbounded();
    let (user_send, user_recv) = unbounded();

    // As `C::Message` must be `Send` but `C::Properties` can't be,
    // we keep two senders but merge them into a single receiver at
    // the task end.
    let channel = Pin::new(Box::new(select(
      user_recv.map(ComponentMessage::Update),
      sys_recv,
    )));

    let type_name = std::any::type_name::<C>();
    let scope = match parent_scope {
      Some(ref p) => p.inherit(type_name, user_send),
      None => Scope::new(type_name, user_send),
    };
    let state = C::create(props);
    let cloned_state = state.clone();
    let context = ViewContext::new(scope.clone());
    let initial_view = cloned_state.view(&context);
    let ui_state = VState::build_root(&initial_view, parent, &scope);

    PartialComponentTask {
      task: ComponentTask {
        scope,
        parent_scope: parent_scope.cloned(),
        state,
        ui_state: Some(ui_state),
        channel,
      },
      // view: initial_view,
      sender: sys_send,
    }
  }

  /// Finalise the partially constructed `ComponentTask` by constructing its
  /// children.
  pub fn finalise(mut self) -> (UnboundedSender<ComponentMessage<C>>, ComponentTask<C, P>) {
    let context: ViewContext<_> = ViewContext::new(self.scope());
    if let Some(ref mut ui_state) = self.task.ui_state {
      let view = &self.task.state.view(&context);
      ui_state.build_children(view, &self.task.scope);
    }

    (self.sender, self.task)
  }

  pub fn object(&self) -> Object {
    self.task.ui_state.as_ref().unwrap().object().clone()
  }

  pub fn scope(&self) -> Scope<C> {
    self.task.scope.clone()
  }
}

#[derive(Default)]
struct LocalContext {
  parent_scope: Option<AnyScope>,
  current_object: Option<WeakRef<Object>>,
}

thread_local! {
    static LOCAL_CONTEXT: RwLock<LocalContext> = RwLock::new(Default::default())
}

impl<C, P> Future for ComponentTask<C, P>
where
  C: 'static + Component,
  P: 'static + Component,
{
  type Output = ();

  fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
    LOCAL_CONTEXT.with(|key| {
      *key.write().unwrap() = LocalContext {
        parent_scope: self.parent_scope.as_ref().map(|scope| scope.clone().into()),
        current_object: self
          .ui_state
          .as_ref()
          .map(|state| state.object().downgrade()),
      };
    });
    let polled = self.get_mut().process(ctx);
    LOCAL_CONTEXT.with(|key| {
      *key.write().unwrap() = Default::default();
    });
    polled
  }
}
