use adw::glib::{
  object::{IsA, ObjectExt},
  Object, Type,
};
use gtk4::Widget;

use crate::reactive::{component::Component, vnode::VNode};

pub struct VObject<C: Component> {
  pub object_type: Type,
  pub constructor: Option<Box<dyn Fn() -> Object>>,
  // pub props: Vec<VProperty>,
  // pub handlers: Vec<VHandler<Model>>,
  pub children: Option<Vec<VNode<C>>>,
}

pub trait VObjectNodeBuilder<C: Component> {
  fn c() -> VNode<C>;
}

impl<T: IsA<Widget>, C: Component> VObjectNodeBuilder<C> for T {
  fn c() -> VNode<C> {
    VNode::of_object(VObject {
      object_type: Self::static_type(),
      constructor: None,
      children: None,
    })
  }
}
