use std::{any::TypeId, marker::PhantomData};

use adw::glib::{object::Cast, MainContext, Object};
use futures::channel::mpsc::UnboundedSender;
use gtk4::{prelude::WidgetExt, Widget};

use crate::reactive::{
  component::{Component, ComponentMessage, ComponentTask},
  scope::Scope,
  vnode::vcomponent::VComponent,
};

trait PropertiesReceiver {
  fn update(&self);
  fn unmounting(&self);
}

pub struct VComponentState<Model: Component> {
  parent: PhantomData<Model>,
  pub object: Object,
  model_type: TypeId,
  state: Box<dyn PropertiesReceiver>,
}

impl<C: 'static + Component> VComponentState<C> {
  pub fn build<Child: 'static + Component>(parent: Option<&Object>, scope: &Scope<C>) -> Self {
    let (sub_state, object) = VSubcomponentState::<Child>::new(parent, scope);
    VComponentState {
      parent: PhantomData,
      object,
      model_type: TypeId::of::<Child>(),
      state: Box::new(sub_state),
    }
  }

  pub fn patch(
    &mut self,
    spec: &VComponent<C>,
    parent: Option<&Object>,
    _scope: &Scope<C>,
  ) -> bool {
    if self.model_type == spec.model_type {
      // Components have same type; update props
      // for prop in &spec.child_props {
      //   (prop.set)(self.object.upcast_ref(), parent, false);
      // }
      self.state.update();
      true
    } else {
      // Component type changed; need to rebuild
      self.state.unmounting();
      false
    }
  }

  pub fn unmount(self) {
    self.state.unmounting();
  }
}

pub struct VSubcomponentState<C: Component> {
  channel: UnboundedSender<ComponentMessage<C>>,
}

impl<C: 'static + Component> VSubcomponentState<C> {
  fn new<P: 'static + Component>(
    parent: Option<&Object>,
    parent_scope: &Scope<P>,
  ) -> (Self, Object) {
    // let props: Model::Properties = props.unwrap();
    let (channel, task) = ComponentTask::<C, P>::new(parent, Some(parent_scope));
    let object = task.object().unwrap();
    // for prop in child_props {
    //   (prop.set)(object.upcast_ref(), parent, true);
    // }
    MainContext::ref_thread_default().spawn_local(task);
    (VSubcomponentState { channel }, object)
  }
}

impl<Model: 'static + Component> PropertiesReceiver for VSubcomponentState<Model> {
  fn update(&self) {
    // let props = raw_props.unwrap();
    // self
    //   .channel
    //   .unbounded_send(ComponentMessage::Props(props))
    //   .expect("failed to send props message over system channel")
  }

  fn unmounting(&self) {
    self
      .channel
      .unbounded_send(ComponentMessage::Unmounted)
      .expect("failed to send unmount message over system channel")
  }
}
