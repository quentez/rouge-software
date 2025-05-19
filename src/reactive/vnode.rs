mod vcomponent;
mod vobject;

use crate::reactive::component::Component;
use vcomponent::VComponent;
use vobject::VObject;

pub enum VNode<C: Component> {
  Object(VObject<C>),
  Component(VComponent<C>),
}
