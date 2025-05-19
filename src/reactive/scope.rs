use std::sync::{atomic::AtomicUsize, Arc};

use futures::channel::mpsc::UnboundedSender;

use crate::reactive::component::Component;

pub struct Scope<C: Component> {
  name: &'static str,
  muted: Arc<AtomicUsize>,
  channel: UnboundedSender<C::Msg>,
}
