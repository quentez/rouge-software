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
  fn of_object(source: VObject<C>) -> VNode<C> {
    VNode::Object(source)
  }

  fn of_component(source: VComponent<C>) -> VNode<C> {
    VNode::Component(source)
  }
}
