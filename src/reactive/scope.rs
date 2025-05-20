use std::sync::{atomic::AtomicUsize, Arc};

use futures::channel::mpsc::UnboundedSender;

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
  pub(crate) fn inherit<Child: Component>(
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
