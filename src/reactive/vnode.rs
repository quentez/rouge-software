pub mod vcomponent;
pub mod vobject;
pub mod vprops;

use vcomponent::VComponent;
use vobject::VObject;

use super::component::Component;

pub enum VNode<'a, C: Component> {
  Object(VObject<'a, C>),
  Component(VComponent<C>),
}

impl<'a, C: Component> VNode<'a, C> {
  pub fn children(self, children: Vec<VNode<'a, C>>) -> Self {
    match self {
      VNode::Object(node) => node.children(children),
      VNode::Component(_) => panic!("Not implemented."),
    }
  }
}
