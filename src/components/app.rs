use adw::{Application, HeaderBar, Window};
use gtk4::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use gtk4::{Box, Button, Label, Orientation};

use crate::components::counter::Counter;
use crate::reactive::component::{UpdateAction, ViewContext};
use crate::reactive::helpers::widget_ext::ReactiveWidgetExt;
use crate::reactive::vnode::vcomponent::VComponentBuilder;
use crate::reactive::vnode::vobject::VObjectBuilder;
use crate::reactive::{component::Component, vnode::VNode};

//
// State.
//

#[derive(Clone, Debug)]
pub struct App {
  count: u8,
}

#[derive(Clone, Debug)]
pub enum AppMessage {
  Increment,
  Decrement,
  Add(i8),
}

impl Default for App {
  fn default() -> Self {
    App { count: 1 }
  }
}

//
// Component.
//

impl Component for App {
  type Message = AppMessage;
  type Props = ();

  fn update(&mut self, message: Self::Message) -> UpdateAction {
    match message {
      AppMessage::Increment => {
        self.count = self.count.saturating_add(1);
        UpdateAction::Render
      }
      AppMessage::Decrement => {
        self.count = self.count.saturating_sub(1);
        UpdateAction::Render
      }
      AppMessage::Add(delta) => {
        self.count = self.count.saturating_add_signed(delta);
        UpdateAction::Render
      }
    }
  }

  fn view(&self, c: &ViewContext<Self>) -> VNode<App> {
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
              w.set_label("Add");
              vec![w.connect_clicked(c.d(|_| AppMessage::Increment))]
            }),
            Button::ce(|w, c| {
              w.set_label("Remove");
              vec![w.connect_clicked(c.d(|_| AppMessage::Decrement))]
            }),
            Counter::cp(Counter {
              name: format!("Counter - {}", self.count),
              count: self.count,
              on_changed: c.d(AppMessage::Add),
            }),
            Counter::cp(Counter {
              name: format!("Count - {}", self.count + 1),
              count: self.count,
              on_changed: c.d(AppMessage::Add),
            }),
          ]),
        ]),
      ]),
    ])
  }
}
