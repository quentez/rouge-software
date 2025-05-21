use std::{
  any::TypeId,
  sync::{
    atomic::{AtomicPtr, AtomicUsize, Ordering},
    Arc,
  },
};

use colored::Colorize;
use futures::channel::mpsc::UnboundedSender;
use log::debug;

use crate::reactive::component::Component;

pub struct Scope<C: Component> {
  name: &'static str,
  muted: Arc<AtomicUsize>,
  channel: UnboundedSender<C::Msg>,
}

impl<C: Component> Scope<C> {
  pub(crate) fn new(name: &'static str, channel: UnboundedSender<C::Msg>) -> Self {
    Scope {
      name,
      muted: Default::default(),
      channel,
    }
  }
}

impl<C: 'static + Component> Scope<C> {
  pub fn inherit<Child: Component>(
    &self,
    name: &'static str,
    channel: UnboundedSender<Child::Msg>,
  ) -> Scope<Child> {
    Scope {
      name,
      muted: self.muted.clone(),
      channel,
    }
  }

  pub fn is_muted(&self) -> bool {
    self.muted.load(Ordering::SeqCst) > 0
  }

  pub fn mute(&self) {
    self.muted.fetch_add(1, Ordering::SeqCst);
  }

  pub fn unmute(&self) {
    self.muted.fetch_sub(1, Ordering::SeqCst);
  }

  pub fn send_message(&self, message: C::Msg) {
    self.log(&message);
    if !self.is_muted() {
      self
        .channel
        .unbounded_send(message)
        .expect("channel has gone unexpectedly out of scope!");
    }
  }

  #[inline(always)]
  fn log(&self, message: &C::Msg) {
    debug!(
      "{} {}: {}",
      format!(
        "Scope::send_message{}",
        if self.is_muted() { " [muted]" } else { "" }
      )
      .green(),
      self.name.magenta().bold(),
      format!("{:?}", message).bright_white().bold()
    );
  }

  pub fn name(&self) -> &'static str {
    &self.name
  }
}

impl<C: Component> Clone for Scope<C> {
  fn clone(&self) -> Self {
    Scope {
      name: self.name,
      muted: self.muted.clone(),
      channel: self.channel.clone(),
    }
  }
}
pub struct AnyScope {
  // type_id: TypeId,
  ptr: AtomicPtr<()>,
  drop: Box<dyn Fn(&mut AtomicPtr<()>) + Send>,
}

impl<C: 'static + Component> From<Scope<C>> for AnyScope {
  fn from(scope: Scope<C>) -> Self {
    let ptr = AtomicPtr::new(Box::into_raw(Box::new(scope)) as *mut ());
    let drop = |ptr: &mut AtomicPtr<()>| {
      let ptr = ptr.swap(std::ptr::null_mut(), Ordering::SeqCst);
      if !ptr.is_null() {
        #[allow(unsafe_code)]
        let scope = unsafe { Box::from_raw(ptr as *mut Scope<C>) };
        std::mem::drop(scope)
      }
    };
    AnyScope {
      // type_id: TypeId::of::<C::Properties>(),
      ptr,
      drop: Box::new(drop),
    }
  }
}
