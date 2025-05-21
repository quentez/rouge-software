use adw::glib::{object::Cast, Object};
use gtk4::Widget;
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

  #[must_use]
  pub(crate) fn patch(
    &mut self,
    vnode: &VNode<Model>,
    parent: Option<&Object>,
    scope: &Scope<Model>,
  ) -> bool {
    match vnode {
      VNode::Object(object) => match self {
        VState::Object(state) => state.patch(object, parent, scope),
        VState::Component(_) => false,
      },
      VNode::Component(vcomp) => match self {
        VState::Component(state) => state.patch(vcomp, parent, scope),
        VState::Object(_) => false,
      },
    }
  }

  pub fn unmount(self) {
    match self {
      VState::Object(state) => state.unmount(),
      VState::Component(state) => state.unmount(),
    }
  }

  pub fn object(&self) -> &Object {
    match self {
      VState::Object(state) => &state.object,
      VState::Component(state) => &state.object,
    }
  }

  pub fn widget(&self) -> Option<&Widget> {
    match self {
      VState::Object(state) => state.object.downcast_ref::<Widget>(),
      VState::Component(state) => state.object.downcast_ref::<Widget>(),
    }
  }
}
