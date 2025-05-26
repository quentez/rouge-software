use adw::{Application, HeaderBar, Window};
use gtk4::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use gtk4::{Box, Button, Label, Orientation, ScrolledWindow};

use crate::components::counter::Counter;
use crate::reactive::component::{UpdateAction, ViewContext};
use crate::reactive::helpers::widget_ext::ReactiveWidgetExt;
use crate::reactive::vnode::vcomponent::VComponentBuilder;
use crate::reactive::vnode::vobject::VObjectBuilder;
use crate::reactive::{component::Component, vnode::VNode};
use crate::services::flatpak::{self, FpRef};

//
// State.
//

#[derive(Clone, Debug, Default)]
pub struct App {
  count: u8,
  refs: Vec<FpRef>,
}

#[derive(Clone, Debug)]
pub enum AppMessage {
  Increment,
  Decrement,
  Add(i8),
}

//
// Component.
//

impl Component for App {
  type Message = AppMessage;
  type Props = ();

  fn create(_: Self::Props) -> Self {
    // List installed apps.
    let refs = flatpak::list().expect("Error listing flatpaks.");

    // let refs: Vec<FpRef> = vec![FpRef {
    //   name: "Hello".to_string(),
    //   summary: "Hello".to_string(),
    //   version: "Hello".to_string(),
    // }];

    App { count: 0, refs }
  }

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
    let items: Vec<VNode<Self>> = self
      .refs
      .iter()
      .map(|r| {
        Box::c(|w| {
          w.set_orientation(Orientation::Horizontal);
          w.set_spacing(10);
          w.set_margin_all(10);
        })
        .children(vec![
          //
          Label::c(|w| {
            w.set_label(&r.name);
            if self.count % 2 == 0 {
              w.add_css_class("accent");
            }
          }),
          Label::c(|w| {
            w.set_label(&r.summary);
          }),
          Label::c(|w| {
            w.set_label(&r.version);
          }),
        ])
      })
      .collect();

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
          Button::ce(|w, c| {
            w.set_label("Add");
            vec![w.connect_clicked(c.d(|_| AppMessage::Increment))]
          }),
          Button::ce(|w, c| {
            w.set_label("Remove");
            vec![w.connect_clicked(c.d(|_| AppMessage::Decrement))]
          }),
          Label::c(|w| {
            w.set_label(&format!("Count: {}", self.count));
          }),
          //
          ScrolledWindow::c(|w| {
            w.set_vexpand(true);
          })
          .children(vec![
            //
            Box::c(|w| {
              w.set_orientation(Orientation::Vertical);
              w.set_spacing(5);
              w.set_margin_all(5);
            })
            .children(items),
          ]),
        ]),
      ]),
    ])
  }
}
