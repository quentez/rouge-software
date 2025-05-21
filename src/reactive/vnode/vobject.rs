use adw::glib::{
  object::{Cast, IsA, ObjectExt},
  Object, Type,
};
use gtk4::Widget;

use crate::reactive::{component::Component, vnode::VNode};

pub struct VObject<C: Component> {
  pub object_type: Type,
  pub constructor: Option<Box<dyn Fn() -> Object>>,
  // pub props: Vec<VProperty>,
  // pub handlers: Vec<VHandler<Model>>,
  pub patcher: Box<dyn Fn(&Object)>,
  pub children: Vec<VNode<C>>,
}

impl<C: Component> VObject<C> {
  pub fn children(self, children: Vec<VNode<C>>) -> VNode<C> {
    VNode::Object(Self { children, ..self })
  }
}

pub trait VObjectBuilder<T: IsA<Object>, C: Component> {
  fn c<P: 'static + Fn(&T)>(patcher: P) -> VNode<C>;
  fn cs() -> VNode<C>;
}

impl<T: IsA<Object>, C: Component> VObjectBuilder<T, C> for T {
  fn c<P: 'static + Fn(&T)>(patcher: P) -> VNode<C> {
    let wrapped_patcher = Box::new(move |obj: &Object| {
      let casted = obj.downcast_ref::<T>().expect("Bad object.");
      println!("Calling patcher.");
      patcher(casted);
    });

    VNode::Object(VObject {
      object_type: Self::static_type(),
      constructor: None,
      patcher: wrapped_patcher,
      children: vec![],
    })
  }

  fn cs() -> VNode<C> {
    let patcher = Box::new(move |_: &Object| {});
    VNode::Object(VObject {
      object_type: Self::static_type(),
      constructor: None,
      patcher,
      children: vec![],
    })
  }
}
