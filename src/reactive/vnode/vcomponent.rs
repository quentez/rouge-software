use std::{any::TypeId, marker::PhantomData};

use adw::glib::{object::IsA, Object};
use gtk4::Widget;

use crate::reactive::{
  component::Component, scope::Scope, vstate::vcomponent_state::VComponentState,
};

use super::{vprops::VProps, VNode};

type Constructor<Model> = dyn Fn(&VProps, Option<&Object>, &Scope<Model>) -> VComponentState<Model>;

pub struct VComponent<C: Component> {
  parent: PhantomData<C>,
  pub model_type: TypeId,
  pub props: VProps,
  pub constructor: Box<Constructor<C>>,
}

pub trait VComponentBuilder<'a, Child: Component> {
  fn c<Parent: 'static + Component>() -> VNode<'a, Parent>;
  fn cp<Parent: 'static + Component>(props: Child::Props) -> VNode<'a, Parent>;
}

impl<'a, Child: 'static + Component> VComponentBuilder<'a, Child> for Child {
  fn c<Parent: 'static + Component>() -> VNode<'a, Parent> {
    let constructor: Box<Constructor<Parent>> = Box::new(VComponentState::build::<Child>);
    VNode::Component(VComponent {
      parent: PhantomData,
      model_type: TypeId::of::<Child>(),
      props: VProps::new(Child::Props::default()),
      constructor,
    })
  }

  fn cp<Parent: 'static + Component>(props: Child::Props) -> VNode<'a, Parent> {
    let constructor: Box<Constructor<Parent>> = Box::new(VComponentState::build::<Child>);
    VNode::Component(VComponent {
      parent: PhantomData,
      model_type: TypeId::of::<Child>(),
      props: VProps::new(props),
      constructor,
    })
  }
}
