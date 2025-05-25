use std::{any::TypeId, marker::PhantomData};

use adw::glib::{object::IsA, Object};
use gtk4::Widget;

use crate::reactive::{
  component::Component, scope::Scope, vstate::vcomponent_state::VComponentState,
};

use super::VNode;

type Constructor<Model> = dyn Fn(Option<&Object>, &Scope<Model>) -> VComponentState<Model>;

pub struct VComponent<C: Component> {
  parent: PhantomData<C>,
  pub model_type: TypeId,
  pub constructor: Box<Constructor<C>>,
}

pub trait VComponentBuilder<'a, C: Component> {
  fn c() -> VNode<'a, C>;
}

impl<'a, C: 'static + Component> VComponentBuilder<'a, C> for C {
  fn c() -> super::VNode<'a, C> {
    let constructor: Box<Constructor<C>> = Box::new(VComponentState::build::<C>);
    VNode::Component(VComponent {
      parent: PhantomData,
      model_type: TypeId::of::<C>(),
      constructor,
    })
  }
}
