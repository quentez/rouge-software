use adw::glib::{
  object::{Cast, IsA},
  Object, SignalHandlerId, Type,
};

use crate::reactive::{component::Component, scope::Scope, vnode::VNode};

pub struct VObjectContext<C: Component> {
  scope: Scope<C>,
}

impl<C: Component> VObjectContext<C> {
  pub fn new(scope: Scope<C>) -> VObjectContext<C> {
    VObjectContext { scope }
  }
}

impl<C: 'static + Component> VObjectContext<C> {
  pub fn d<W: IsA<Object>, MB: 'static + Fn(&W) -> C::Message>(
    &self,
    message_builder: MB,
  ) -> impl 'static + Fn(&W) {
    let scope_clone = self.scope.clone();
    move |o| {
      let message = message_builder(o);
      scope_clone.send_message(message);
    }
  }
}

pub struct VObject<'a, C: Component> {
  pub object_type: Type,
  pub constructor: Option<Box<dyn Fn() -> Object>>,
  pub patcher: Box<dyn 'a + Fn(&Object, &VObjectContext<C>) -> Vec<SignalHandlerId>>,
  pub children: Vec<VNode<'a, C>>,
  // pub props: Vec<VProperty>,
  // pub handlers: Vec<VHandler<Model>>,
}

impl<'a, C: Component> VObject<'a, C> {
  pub fn children(self, children: Vec<VNode<'a, C>>) -> VNode<'a, C> {
    VNode::Object(Self { children, ..self })
  }
}

type MessageBuilder<W, M> = Box<dyn Fn(&W) -> M + 'static>;
type Dispatch<W, M> = Box<dyn Fn(MessageBuilder<W, M>) -> Box<dyn Fn(&W) + 'static> + 'static>;

pub trait VObjectBuilder<'a, W: IsA<Object>, C: Component> {
  fn ce<P: 'a + Fn(&W, &VObjectContext<C>) -> Vec<SignalHandlerId>>(patcher: P) -> VNode<'a, C>;
  fn c<P: 'a + Fn(&W)>(patcher: P) -> VNode<'a, C>;
  fn cs() -> VNode<'a, C>;
}

impl<'a, W: IsA<Object>, C: Component> VObjectBuilder<'a, W, C> for W {
  fn ce<P: 'a + Fn(&W, &VObjectContext<C>) -> Vec<SignalHandlerId>>(patcher: P) -> VNode<'a, C> {
    let wrapped_patcher = Box::new(move |obj: &Object, context: &VObjectContext<C>| {
      let casted = obj.downcast_ref::<W>().expect("Bad object.");
      patcher(casted, context)
    });

    VNode::Object(VObject {
      object_type: Self::static_type(),
      constructor: None,
      patcher: wrapped_patcher,
      children: vec![],
    })
  }

  fn c<P: 'a + Fn(&W)>(patcher: P) -> VNode<'a, C> {
    let wrapped_patcher = Box::new(move |obj: &Object, _: &VObjectContext<C>| {
      let casted = obj.downcast_ref::<W>().expect("Bad object.");
      patcher(casted);
      vec![]
    });

    VNode::Object(VObject {
      object_type: Self::static_type(),
      constructor: None,
      patcher: wrapped_patcher,
      children: vec![],
    })
  }

  fn cs() -> VNode<'a, C> {
    let patcher = Box::new(move |_: &Object, _: &VObjectContext<C>| vec![]);
    VNode::Object(VObject {
      object_type: Self::static_type(),
      constructor: None,
      patcher,
      children: vec![],
    })
  }
}
