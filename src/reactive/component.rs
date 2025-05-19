use std::fmt::Debug;

use crate::reactive::vnode::VNode;

pub enum UpdateAction {
  None,
  Render,
}

pub trait Component: Default + Unpin {
  type Msg: Clone + Send + Debug + Unpin;
  type Props: Clone + Default;

  fn update(&self, _message: Self::Msg) -> UpdateAction {
    UpdateAction::None
  }

  fn create(_props: Self::Props) -> Self {
    Default::default()
  }

  fn change(&self, _props: Self::Props) -> UpdateAction {
    unimplemented!("Add a Component::change() implementation.")
  }

  fn mounted(&self) {}
  fn unmounted(&self) {}

  fn view(&self) -> VNode;
}
