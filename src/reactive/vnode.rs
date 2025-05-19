pub mod vcomponent;
pub mod vobject;

use vcomponent::VComponent;
use vobject::VObject;

pub enum VNode {
  Object(VObject),
  Component(VComponent),
}

impl VNode {
  fn of_object(source: VObject) -> VNode {
    VNode::Object(source)
  }

  fn of_component(source: VComponent) -> VNode {
    VNode::Component(source)
  }
}
