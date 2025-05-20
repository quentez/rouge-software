use std::{any::TypeId, marker::PhantomData};

use adw::glib::Object;

use crate::reactive::component::Component;

trait PropertiesReceiver {
  fn update(&self);
  fn unmounting(&self);
}

pub struct VComponentState<Model: Component> {
  parent: PhantomData<Model>,
  pub object: Object,
  model_type: TypeId,
  state: Box<dyn PropertiesReceiver>,
}
