use adw::{Application, HeaderBar, Window};
use gtk4::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use gtk4::{Box, Button, Label, Orientation};

use crate::components::counter::Counter;
use crate::reactive::component::UpdateAction;
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
pub enum AppMsg {
  Increment,
  Decrement,
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

  fn view(&self) -> VNode<App> {
    let controls: Vec<VNode<'_, _>> = (0..(self.count + 1))
      .flat_map(|_| -> Vec<VNode<'_, _>> {
        vec![
          //
          Counter::c(),
        ]
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
          Box::c(|w| {
            w.set_orientation(Orientation::Vertical);
            w.set_spacing(5);
            w.set_margin_all(5);
          })
          .children(controls),
        ]),
      ]),
    ])
  }
}
