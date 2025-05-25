use adw::{Application, HeaderBar, Window};
use gtk4::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use gtk4::{Box, Button, Label, Orientation};

use crate::reactive::component::UpdateAction;
use crate::reactive::helpers::widget_ext::ReactiveWidgetExt;
use crate::reactive::vnode::vobject::VObjectBuilder;
use crate::reactive::{component::Component, vnode::VNode};
use std::boxed::Box as B;

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

  fn update(&mut self, message: Self::Msg) -> UpdateAction {
    match message {
      AppMsg::Increment => {
        self.count = self.count.saturating_add(1);
        UpdateAction::Render
      }
      AppMsg::Decrement => {
        self.count = self.count.saturating_sub(1);
        UpdateAction::Render
      }
    }
  }

  fn view(&self) -> VNode<AppModel> {
    Application::cs().children(vec![
      //
      Window::c(|w| {
        w.set_default_width(300);
        w.set_default_height(100);
      })
      .children(vec![
        //
        Box::c(|w| {
          w.set_orientation(Orientation::Vertical);
        })
        .children(vec![
          //
          HeaderBar::c(|w| {
            w.set_title_widget(Some(&Label::new(Some("My Adwaita App"))));
          }),
          Box::c(|w| {
            w.set_orientation(Orientation::Vertical);
            w.set_spacing(5);
            w.set_margin_all(5);
          })
          .children(vec![
            //
            Button::ce(|w, c| {
              w.set_label("Increment");
              vec![w.connect_clicked(c.d(|_| AppMsg::Increment))]
            }),
            Button::ce(|w, c| {
              w.set_label("Decrement");
              vec![w.connect_clicked(c.d(|_| AppMsg::Decrement))]
            }),
            Label::c(|w| {
              w.set_label(&format!("Counter: {}", self.count));
              w.set_margin_all(5);
            }),
          ]),
        ]),
      ]),
    ])
  }
}
