use crate::reactive::component::UpdateAction;
use crate::reactive::helpers::widget_ext::ReactiveWidgetExt;
use crate::reactive::vnode::vobject::VObjectBuilder;
use crate::reactive::{component::Component, vnode::VNode};
use gtk4::prelude::{BoxExt, ButtonExt, OrientableExt};
use gtk4::{Box, Button, Label, Orientation};

//
// State.
//

#[derive(Clone, Debug)]
pub struct Counter {
  count: u8,
}

#[derive(Clone, Debug)]
pub enum CounterMessage {
  Increment,
  Decrement,
}

impl Default for Counter {
  fn default() -> Self {
    Counter { count: 1 }
  }
}

//
// Component.
//

impl Component for Counter {
  type Msg = CounterMessage;
  type Props = ();

  fn update(&mut self, message: Self::Msg) -> UpdateAction {
    match message {
      CounterMessage::Increment => {
        self.count = self.count.saturating_add(1);
        UpdateAction::Render
      }
      CounterMessage::Decrement => {
        self.count = self.count.saturating_sub(1);
        UpdateAction::Render
      }
    }
  }

  fn view(&self) -> VNode<Self> {
    Box::c(|w| {
      w.set_orientation(Orientation::Vertical);
      w.set_spacing(5);
      w.set_margin_all(5);
    })
    .children(vec![
      //
      Button::ce(|w, c| {
        w.set_label("Increment");
        vec![w.connect_clicked(c.d(|_| CounterMessage::Increment))]
      }),
      Button::ce(|w, c| {
        w.set_label("Decrement");
        vec![w.connect_clicked(c.d(|_| CounterMessage::Decrement))]
      }),
      Label::c(|w| {
        w.set_label(&format!("Counter: {}", self.count));
        w.set_margin_all(5);
      }),
    ])
  }
}
