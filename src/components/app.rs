use crate::reactive::vnode::vobject::VObjectBuilder;
use crate::reactive::{component::Component, vnode::VNode};
use adw::*;

//
// State.
//

#[derive(Clone, Debug)]
pub struct AppModel {
  count: u8,
}

#[derive(Clone, Debug)]
pub enum AppMsg {
  Increment,
  Decrement,
}

impl Default for AppModel {
  fn default() -> Self {
    AppModel { count: 1 }
  }
}

//
// Component.
//

impl Component for AppModel {
  type Msg = AppMsg;
  type Props = ();

  fn view(&self) -> VNode<AppModel> {
    Application::c().children(vec![Window::c()])
  }
}
