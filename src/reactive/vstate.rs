use adw::glib::Object;
use vcomponent_state::VComponentState;
use vobject_state::VObjectState;

use crate::reactive::component::Component;

use super::{scope::Scope, vnode::VNode};

pub mod vcomponent_state;
pub mod vobject_state;

pub enum VState<Model: Component> {
  Object(VObjectState<Model>),
  Component(VComponentState<Model>),
}

impl<Model: 'static + Component> VState<Model> {
  pub fn build(vnode: &VNode<Model>, parent: Option<&Object>, scope: &Scope<Model>) -> Self {
    match vnode {
      VNode::Object(object) => VState::Object(VObjectState::build(object, parent, scope)),
      VNode::Component(vcomp) => {
        let comp = (vcomp.constructor)(parent, scope);
        VState::Component(comp)
      }
    }
  }

  /// Build a full state from a `VItem` spec.
  pub fn build_root(vnode: &VNode<Model>, parent: Option<&Object>, scope: &Scope<Model>) -> Self {
    match vnode {
      VNode::Object(object) => VState::Object(VObjectState::build_root(object, parent, scope)),
      VNode::Component(_vcomp) => {
        // let comp = (vcomp.constructor)(&vcomp.props, parent, &vcomp.child_props, scope);
        // State::Component(comp)
        unimplemented!()
      }
    }
  }

  pub fn build_children(&mut self, vnode: &VNode<Model>, scope: &Scope<Model>) {
    match vnode {
      VNode::Object(vobject) => match self {
        VState::Object(gtk_state) => gtk_state.build_children(vobject, scope),
        _ => unimplemented!(),
      },
      _ => unimplemented!(),
    }
  }

  pub fn object(&self) -> &Object {
    match self {
      VState::Object(state) => &state.object,
      VState::Component(state) => &state.object,
    }
  }
}
