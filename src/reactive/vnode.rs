pub mod vcomponent;
pub mod vobject;

use vcomponent::VComponent;
use vobject::VObject;

use super::component::Component;

pub enum VNode<C: Component> {
  Object(VObject<C>),
  Component(VComponent<C>),
}

impl<C: Component> VNode<C> {
  pub fn children(self, children: Vec<VNode<C>>) -> Self {
    match self {
      VNode::Object(node) => node.children(children),
      VNode::Component(_) => panic!("Not implemented."),
    }
  }
}
