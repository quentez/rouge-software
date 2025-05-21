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
  pub children: Vec<VNode<C>>,
}

impl<C: Component> VObject<C> {
  pub fn children(self, children: Vec<VNode<C>>) -> VNode<C> {
    VNode::Object(Self { children, ..self })
  }
}

pub trait VObjectBuilder<C: Component> {
  fn c() -> VNode<C>;
}

impl<T: IsA<Object>, C: Component> VObjectBuilder<C> for T {
  fn c() -> VNode<C> {
    VNode::Object(VObject {
      object_type: Self::static_type(),
      constructor: None,
      children: vec![],
    })
  }
}
