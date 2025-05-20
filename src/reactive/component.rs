use std::fmt::Debug;
use std::pin::Pin;

use adw::glib::Object;
use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::stream::{select, Stream};
use futures::StreamExt;

use crate::reactive::scope::Scope;
use crate::reactive::vnode::VNode;

use super::vstate::VState;

pub enum UpdateAction {
  None,
  Render,
}

pub trait Component: Default + Unpin {
  type Msg: Clone + Send + Debug + Unpin;
  type Props: Clone + Default;

  fn update(&self, _message: Self::Msg) -> UpdateAction {
    UpdateAction::None
  }

  fn create(_props: Self::Props) -> Self {
    Default::default()
  }

  fn change(&self, _props: Self::Props) -> UpdateAction {
    unimplemented!("Add a Component::change() implementation.")
  }

  fn mounted(&self) {}
  fn unmounted(&self) {}

  fn view(&self) -> VNode<Self>;
}

pub enum ComponentMessage<C: Component> {
  Update(C::Msg),
  Props(C::Props),
  Mounted,
  Unmounted,
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

pub struct PartialComponentTask<C, P>
where
  C: Component,
  P: Component,
{
  task: ComponentTask<C, P>,
  view: VNode<C>,
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
    let initial_view = state.view();
    let ui_state = VState::build_root(&initial_view, parent, &scope);
    PartialComponentTask {
      task: ComponentTask {
        scope,
        parent_scope: parent_scope.cloned(),
        state,
        ui_state: Some(ui_state),
        channel,
      },
      view: initial_view,
      sender: sys_send,
    }
  }

  /// Finalise the partially constructed `ComponentTask` by constructing its
  /// children.
  pub fn finalise(mut self) -> (UnboundedSender<ComponentMessage<C>>, ComponentTask<C, P>) {
    if let Some(ref mut ui_state) = self.task.ui_state {
      ui_state.build_children(&self.view, &self.task.scope);
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
