// use adw::gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
// use relm4::{adw, gtk, view, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

// pub struct CounterModel {
//   count: u8,
// }

// #[derive(Debug)]
// pub enum CounterMessage {
//   Increment,
//   Decrement,
// }

// #[relm4::component(pub)]
// impl SimpleComponent for CounterModel {
//   type Input = CounterMessage;
//   type Output = ();

//   type Init = u8;

//   view! {
//     gtk::Window {
//       set_title: Some("Software"),
//       set_default_width: 300,
//       set_default_height: 100,

//       gtk::Box {
//         set_orientation: gtk::Orientation::Vertical,
//         set_spacing: 5,
//         set_margin_all: 5,

//         gtk::Button {
//           set_label: "Increment",
//           connect_clicked => CounterMessage::Increment
//         },

//         gtk::Button {
//           set_label: "Decrement",
//           connect_clicked => CounterMessage::Decrement,
//         },

//         gtk::Label {
//           #[watch]
//           set_label: &format!("Counter: {}", model.count),
//           set_margin_all: 20,
//         }
//       }
//     }
//   }

//   fn init(
//     count: Self::Init,
//     root: Self::Root,
//     sender: ComponentSender<Self>,
//   ) -> ComponentParts<Self> {
//     let model = CounterModel { count };
//     let widgets = view_output!();

//     ComponentParts { model, widgets }
//   }

//   fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
//     match message {
//       CounterMessage::Increment => {
//         self.count = self.count.wrapping_add(1);
//       }
//       CounterMessage::Decrement => {
//         self.count = self.count.wrapping_sub(1);
//       }
//     }
//   }
// }
