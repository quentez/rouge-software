use std::{any::TypeId, marker::PhantomData};

use adw::glib::Object;

use crate::reactive::{
  component::Component, scope::Scope, vstate::vcomponent_state::VComponentState,
};

type Constructor<Model> = dyn Fn(Option<&Object>, &Scope<Model>) -> VComponentState<Model>;

pub struct VComponent<C: Component> {
  parent: PhantomData<C>,
  pub model_type: TypeId,
  pub constructor: Box<Constructor<C>>,
}
