use adw::glib::{
  object::{Cast, IsA},
  Object, Type,
};

use crate::reactive::{component::Component, vnode::VNode};

pub struct VObject<C: Component> {
  pub object_type: Type,
  pub constructor: Option<Box<dyn Fn() -> Object>>,
  // pub props: Vec<VProperty>,
  // pub handlers: Vec<VHandler<Model>>,
  pub patcher: Box<dyn Fn(&Object, Box<dyn Fn(C::Msg)>)>,
  pub children: Vec<VNode<C>>,
}

impl<C: Component> VObject<C> {
  pub fn children(self, children: Vec<VNode<C>>) -> VNode<C> {
    VNode::Object(Self { children, ..self })
  }
}

pub trait VObjectBuilder<T: IsA<Object>, C: Component> {
  fn ce<P: 'static + Fn(&T, Box<dyn Fn(C::Msg)>)>(patcher: P) -> VNode<C>;
  fn c<P: 'static + Fn(&T)>(patcher: P) -> VNode<C>;
  fn cs() -> VNode<C>;
}

impl<T: IsA<Object>, C: Component> VObjectBuilder<T, C> for T {
  fn ce<P: 'static + Fn(&T, Box<dyn Fn(C::Msg)>)>(patcher: P) -> VNode<C> {
    let wrapped_patcher = Box::new(move |obj: &Object, dispatch: Box<dyn Fn(C::Msg)>| {
      let casted = obj.downcast_ref::<T>().expect("Bad object.");
      patcher(casted, dispatch);
    });

    VNode::Object(VObject {
      object_type: Self::static_type(),
      constructor: None,
      patcher: wrapped_patcher,
      children: vec![],
    })
  }

  fn c<P: 'static + Fn(&T)>(patcher: P) -> VNode<C> {
    let wrapped_patcher = Box::new(move |obj: &Object, _: Box<dyn Fn(C::Msg)>| {
      let casted = obj.downcast_ref::<T>().expect("Bad object.");
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
    let patcher = Box::new(move |_: &Object, _: Box<dyn Fn(C::Msg)>| {});
    VNode::Object(VObject {
      object_type: Self::static_type(),
      constructor: None,
      patcher,
      children: vec![],
    })
  }
}
