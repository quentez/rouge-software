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
  fn c<Parent: 'static + Component>() -> VNode<'a, Parent>;
}

impl<'a, Child: 'static + Component> VComponentBuilder<'a, Child> for Child {
  fn c<Parent: 'static + Component>() -> super::VNode<'a, Parent> {
    let constructor: Box<Constructor<Parent>> = Box::new(VComponentState::build::<Child>);
    VNode::Component(VComponent {
      parent: PhantomData,
      model_type: TypeId::of::<Child>(),
      constructor,
    })
  }
}
