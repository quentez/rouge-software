use adw::{Application, Spinner, Window};
use gtk4::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use gtk4::{Box, Button, Label, Orientation};

use crate::helpers::widget_ext::ReactiveWidgetExt;
use crate::reactive::vnode::vobject::VObjectBuilder;
use crate::reactive::{component::Component, vnode::VNode};

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
    Application::cs().children(vec![
      //
      Window::c(|c| {
        c.set_title(Some("App"));
        c.set_default_width(300);
        c.set_default_height(100);
      })
      .children(vec![
        //
        Box::c(|c| {
          c.set_orientation(Orientation::Vertical);
          c.set_spacing(5);
          c.set_margin_all(5);
        })
        .children(vec![
          //
          Button::c(|c| {
            c.set_label("Increment");
          }),
          Button::c(|c| {
            c.set_label("Decrement");
          }),
          Label::c(|c| {
            c.set_label("Welcome to the app!");
            c.set_margin_all(5);
          }),
        ]),
      ]),
    ])
  }
}
