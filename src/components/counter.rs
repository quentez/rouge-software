use crate::reactive::callback::Callback;
use crate::reactive::component::{UpdateAction, ViewContext};
use crate::reactive::helpers::widget_ext::ReactiveWidgetExt;
use crate::reactive::vnode::vobject::VObjectBuilder;
use crate::reactive::{component::Component, vnode::VNode};
use gtk4::prelude::{BoxExt, ButtonExt, OrientableExt};
use gtk4::{Box, Button, Label, Orientation};

//
// State.
//

#[derive(Clone, Debug, Default)]
pub struct Counter {
  pub name: String,
  pub count: u8,
  pub on_changed: Callback<i8>,
}

// #[derive(Clone, Debug)]
// pub struct CounterProps {
//   pub name: String,
// }

// impl Default for CounterProps {
//   fn default() -> Self {
//     CounterProps {
//       name: "Counter".to_string(),
//     }
//   }
// }

#[derive(Clone, Debug)]
pub enum CounterMessage {
  Increment,
  Decrement,
}

//
// Component.
//

impl Component for Counter {
  type Message = CounterMessage;
  type Props = Counter;

  fn create(props: Self::Props) -> Self {
    props
    // Self {
    //   name: props.name,
    //   count: 1,
    // }
  }

  fn change(&mut self, props: Self::Props) -> UpdateAction {
    // if self.name == props.name {
    //   UpdateAction::None
    // } else {
    //   self.name = props.name;
    //   UpdateAction::Render
    // }
    *self = props;
    UpdateAction::Render
  }

  fn update(&mut self, message: Self::Message) -> UpdateAction {
    match message {
      CounterMessage::Increment => {
        // self.count = self.count.saturating_add(1);
        // UpdateAction::Render
        self.on_changed.send(1);
      }
      CounterMessage::Decrement => {
        // self.count = self.count.saturating_sub(1);
        // UpdateAction::Render
        self.on_changed.send(-1);
      }
    }
    UpdateAction::None
  }

  fn view(&self, _: &ViewContext<Self>) -> VNode<Self> {
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
        w.set_label(&format!("{}: {}", self.name, self.count));
        w.set_margin_all(5);
      }),
    ])
  }
}
