use adw::glib::{
  object::{IsA, ObjectExt},
  Object, Type,
};
use gtk4::Widget;

use crate::reactive::vnode::VNode;

pub struct VObject {
  pub object_type: Type,
  // pub constructor: Option<Box<dyn Fn() -> Object>>,
  // pub props: Vec<VProperty>,
  // pub handlers: Vec<VHandler<Model>>,
  pub children: Option<Vec<VNode>>,
}

pub trait VObjectNodeBuilder {
  fn c() -> VNode;
}

impl<T: IsA<Widget>> VObjectNodeBuilder for T {
  fn c() -> VNode {
    VNode::of_object(VObject {
      object_type: Self::static_type(),
      // constructor: (),
      children: None,
    })
  }
}

// fn test() -> VNode {
//   adw::AboutDialog::c()
// }
