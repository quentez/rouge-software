use adw::glib::{Object, Type};

use crate::reactive::component::Component;
use crate::reactive::vnode::VNode;

pub struct VObject<C: Component> {
  pub object_type: Type,
  pub constructor: Option<Box<dyn Fn() -> Object>>,
  // pub props: Vec<VProperty>,
  // pub handlers: Vec<VHandler<Model>>,
  pub children: Vec<VNode<C>>,
}
