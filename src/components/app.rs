use adw::{Application, HeaderBar, Window};
use gtk4::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use gtk4::{Box, Button, Label, Orientation};

use crate::helpers::widget_ext::ReactiveWidgetExt;
use crate::reactive::component::UpdateAction;
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
    let count_cloned = self.count;

    Application::cs().children(vec![
      //
      Window::c(|c| {
        c.set_default_width(300);
        c.set_default_height(100);
      })
      .children(vec![
        //
        Box::c(|c| {
          c.set_orientation(Orientation::Vertical);
        })
        .children(vec![
          //
          HeaderBar::c(|c| {
            c.set_title_widget(Some(&Label::new(Some("My Adwaita App"))));
          }),
          Box::c(|c| {
            c.set_orientation(Orientation::Vertical);
            c.set_spacing(5);
            c.set_margin_all(5);
          })
          .children(vec![
            //
            Button::ce(|c, d| {
              c.set_label("Increment");
              c.connect_clicked(move |_| d(AppMsg::Increment));
            }),
            Button::ce(|c, d| {
              c.set_label("Decrement");
              c.connect_clicked(move |_| d(AppMsg::Decrement));
            }),
            Label::c(move |c| {
              c.set_label(&format!("Counter: {}", count_cloned));
              c.set_margin_all(5);
            }),
          ]),
        ]),
      ]),
    ])
  }
}
