use adw::{
  gio::{
    prelude::{ActionExt, ActionMapExt},
    Action, Menu, MenuItem,
  },
  glib::{
    object::{Cast, IsA, ObjectExt},
    Object, SignalHandlerId,
  },
  prelude::BinExt,
  Application, ApplicationWindow, Bin, HeaderBar, Window,
};
use gtk4::{
  prelude::{ApplicationWindowExt, BoxExt, GridExt, GtkApplicationExt, GtkWindowExt, WidgetExt},
  Box, Builder, Grid, Notebook, ShortcutsWindow, Widget,
};
use std::collections::HashMap;

use super::VState;
use crate::reactive::{
  component::Component,
  scope::Scope,
  vnode::{vobject::VObject, VNode},
};

pub struct VObjectState<Model: Component> {
  pub object: Object,
  handlers: HashMap<(&'static str, &'static str), SignalHandlerId>,
  children: Vec<VState<Model>>,
}

fn build_obj<A: IsA<Object>, C: Component>(spec: &VObject<C>) -> A {
  let class = spec.object_type;
  let obj = if let Some(ref cons) = spec.constructor {
    cons()
  } else {
    let mut ui = String::new();
    ui += &format!("<interface><object class=\"{}\"", class);
    ui += "/></interface>";

    let builder = Builder::from_string(&ui);
    let objects = builder.objects();
    objects
      .last()
      .unwrap_or_else(|| panic!("unknown class {}", class))
      .clone()
  };
  obj
    .downcast::<A>()
    .unwrap_or_else(|_| panic!("build_obj: cannot cast {} to {}", class, A::static_type()))
}

fn add_child<Model: Component>(
  parent: &Object,
  index: usize,
  total: usize,
  child_spec: &VNode<Model>,
  child: &Object,
) {
  if let Some(application) = parent.downcast_ref::<Application>() {
    if let Some(window) = child.downcast_ref::<Window>() {
      application.add_window(window);
    } else if let Some(action) = child.downcast_ref::<Action>() {
      application.add_action(action);
    } else {
      panic!(
        "Application's children must be Windows or Actions, but {} was found.",
        child.type_()
      );
    }
  // } else if let Some(button) = parent.downcast_ref::<MenuButton>() {
  //   // MenuButton: can only have a single child, either a `Menu` set with
  //   // `set_popup` or any other `Widget` set with `set_popover`.
  //   if total > 1 {
  //     panic!(
  //       "MenuButton can only have 1 child, but {} were found.",
  //       total,
  //     );
  //   }
  // if let Some(menu) = child.downcast_ref::<Menu>() {
  //   button.set_popup(Some(menu));
  // } else if let Some(widget) = child.downcast_ref::<Widget>() {
  //   button.set_popover(Some(widget));
  // } else {
  //   panic!(
  //     "MenuButton's children must be Widgets, but {} was found.",
  //     child.type_()
  //   );
  // }
  // } else if let Some(item) = parent.downcast_ref::<MenuItem>() {
  //   // MenuItem: single child, must be a `Menu`, set with `set_submenu`.
  //   if total > 1 {
  //     panic!("MenuItem can only have 1 child, but {} were found.", total);
  //   }
  //   if let Some(menu) = child.downcast_ref::<Menu>() {
  //     item.set_submenu(Some(menu));
  //   } else {
  //     panic!(
  //       "MenuItem can only take children of type Menu, but {} was found.",
  //       child.type_()
  //     );
  //   }
  // } else if let Some(dialog) = parent.downcast_ref::<Dialog>() {
  //   // Dialog: children must be added to the Dialog's content area through
  //   // get_content_area().
  //   if let Some(widget) = child.downcast_ref::<Widget>() {
  //     dialog.content_area().add(widget);
  //   } else {
  //     panic!(
  //       "Dialog's children must be Widgets, but {} was found.",
  //       child.type_()s
  //     );
  //   }
  } else if let Some(window) = parent.downcast_ref::<ApplicationWindow>() {
    // ApplicationWindow: takes any number of Actions, optionally one
    // ShortcutsWindow added with `set_help_overlay()`, and either 1 or 2
    // Widgets. If 1, it's the main widget. If 2, the first is added with
    // `set_titlebar()` and the second is the main widget.
    if let Some(action) = child.downcast_ref::<Action>() {
      window.add_action(action);
    } else if let Some(help_overlay) = child.downcast_ref::<ShortcutsWindow>() {
      window.set_help_overlay(Some(help_overlay));
    } else if let Some(widget) = child.downcast_ref::<Widget>() {
      match window.child() {
        None => window.set_child(Some(widget)),
        Some(ref titlebar) if window.titlebar().is_none() => {
          // window.remove(titlebar);
          window.set_titlebar(Some(titlebar));
          window.set_child(Some(widget));
        }
        _ => panic!("ApplicationWindow can have at most two Widget children."),
      }
    } else {
      panic!(
        "ApplicationWindow's children must be Actions or Widgets, but {} was found.",
        child.type_()
      );
    }
  } else if let Some(window) = parent.downcast_ref::<Window>() {
    // Window: takes only 1 or 2 Widgets. If 1 widget child, it's the
    // window's main widget. If 2, the first is the title bar and the second
    // is the main widget. More than 2 goes boom.
    if let Some(widget) = child.downcast_ref::<Widget>() {
      if total == 2 && index == 0 {
        window.set_titlebar(Some(widget));
      } else {
        window.set_child(Some(widget));
      }
    } else {
      panic!(
        "Window's children must be Widgets, but {} was found.",
        child.type_()
      );
    }
  // } else if let Some(parent) = parent.downcast_ref::<Bin>() {
  //   // Bin: can only have a single child.
  //   if total > 1 {
  //     panic!("Bins can only have 1 child, but {} were found.", total);
  //   }
  //   if let Some(widget) = child.downcast_ref::<Widget>() {
  //     parent.set_child(Some(widget));
  //   } else {
  //     panic!(
  //       "Bin's child must be a Widget, but {} was found.",
  //       child.type_()
  //     );
  //   }
  } else if let Some(parent) = parent.downcast_ref::<HeaderBar>() {
    // HeaderBar: added normally, except one widget can be added using
    // set_custom_title if it has the custom_title=true child property
    // (which is faked in ext.rs). More than one child with this property is
    // undefined behaviour.
    if let Some(widget) = child.downcast_ref::<Widget>() {
      parent.pack_end(widget);
      // if child_spec.get_child_prop("custom_title").is_some() {
      //   parent.set_custom_title(Some(widget));
      // } else {
      //   parent.add(widget);
      // }
    } else {
      panic!(
        "HeaderBar's children must be Widgets, but {} was found.",
        child.type_()
      );
    }
  } else if let Some(parent) = parent.downcast_ref::<Box>() {
    // Box: added normally, except one widget can be added using
    // set_center_widget() if it has the center_widget=true child property
    // (which is faked in ext.rs). More than one child with this property is
    // undefined behaviour.
    if let Some(widget) = child.downcast_ref::<Widget>() {
      parent.append(widget);
      // if child_spec.get_child_prop("center_widget").is_some() {
      //   parent.set_center_widget(Some(widget));
      // } else {
      //   parent.add(widget);
      // }
    } else {
      panic!(
        "Box's children must be Widgets, but {} was found.",
        child.type_()
      );
    }
  } else if let Some(parent) = parent.downcast_ref::<Grid>() {
    if let Some(widget) = child.downcast_ref::<Widget>() {
      // by default we put widgets in the top left corner of the grid
      // with row and col span of 1; this would typically get overridden
      // via props but setting the default is important in order to avoid
      // making the user specify these for every single child widget
      parent.attach(widget, 0, 0, 1, 1);
    } else {
      panic!(
        "Grid's children must be Widgets, but {} was found.",
        child.type_()
      );
    }
  // } else if let Some(parent) = parent.downcast_ref::<Notebook>() {
  //   // Notebook: added normally, except one widget can be added using
  //   // set_action_widget if it has the action_widget_start or
  //   // action_widget_end child property (which are faked in ext.rs). More
  //   // than one child with each of these properties is undefined behaviour.
  //   if let Some(widget) = child.downcast_ref::<Widget>() {
  //     if child_spec.get_child_prop("action_widget_start").is_some() {
  //       parent.set_action_widget(widget, gtk::PackType::Start);
  //     } else if child_spec.get_child_prop("action_widget_end").is_some() {
  //       parent.set_action_widget(widget, gtk::PackType::End);
  //     } else {
  //       parent.add(widget);
  //     }
  //   } else {
  //     panic!(
  //       "Notebook's children must be Widgets, but {} was found.",
  //       child.type_()
  //     );
  //   }
  // } else if let Some(container) = parent.downcast_ref::<Container>() {
  //   if let Some(widget) = child.downcast_ref::<Widget>() {
  //     container.add(widget);
  //   } else {
  //     panic!(
  //       "Container's children must be Widgets, but {} was found.",
  //       child.type_()
  //     );
  //   }
  } else {
    panic!("Don't know how to add children to a {}", parent.type_());
  }
  // // Apply child properties
  // for prop in child_spec.get_child_props() {
  //   (prop.set)(child.upcast_ref(), Some(parent), true);
  // }
}

fn remove_child(parent: &Object, child: &Object) {
  // There are also special cases for removing children.
  if let Some(application) = parent.downcast_ref::<Application>() {
    if let Some(window) = child.downcast_ref::<Window>() {
      application.remove_window(window);
    } else if let Some(action) = child.downcast_ref::<Action>() {
      application.remove_action(&action.name());
    } else {
      panic!(
        "Applications can only contain Windows, but was asked to remove a {}.",
        child.type_()
      );
    }
  } else if let Some(widget) = parent.downcast_ref::<Widget>() {
    // For a Container and a Widget child, we should always be able to call
    // `Container::remove`.
    if let Some(child_widget) = child.downcast_ref::<Widget>() {
      widget.remove
      container.remove(child_widget);
    } else {
      panic!(
        "Widgets can only contain other Widgets but was asked to remove a {}.",
        child.type_()
      );
    }
  } else {
    panic!("Don't know how to remove a child from a {}", parent.type_());
  }
}

impl<C: 'static + Component> VObjectState<C> {
  // This function build the root object, but not its children. You must call
  // `build_children()` to finalise construction.
  pub fn build_root(vobj: &VObject<C>, parent: Option<&Object>, scope: &Scope<C>) -> Self {
    // Build this object
    let object: Object = build_obj(&vobj);

    // // Apply properties
    // for prop in &vobj.properties {
    //   (prop.set)(object.upcast_ref(), parent, true);
    // }

    // // Apply handlers
    // let mut handlers = HashMap::new();
    // for handler in &vobj.handlers {
    //   let handle = (handler.set)(object.upcast_ref(), scope);
    //   handlers.insert((handler.name, handler.id), handle);
    // }

    VObjectState {
      object: object.upcast(),
      handlers: HashMap::new(),
      children: Vec::new(),
    }
  }

  pub fn build_children(&mut self, vobj: &VObject<C>, scope: &Scope<C>) {
    let object = &self.object;

    // Build children.
    if let Some(children) = &vobj.children {
      let total_children = children.len();
      for (index, child_spec) in children.iter().enumerate() {
        let child = VState::build(child_spec, Some(&object), &scope);
        let child_object = child.object().clone();
        add_child(&object, index, total_children, child_spec, &child_object);
        self.children.push(child);
      }
    }

    // Show this object, if it's a widget.
    if let Some(widget) = self.object.downcast_ref::<Widget>() {
      widget.set_visible(true);
    }
  }

  pub fn build(vobj: &VObject<C>, parent: Option<&Object>, scope: &Scope<C>) -> Self {
    let mut state = Self::build_root(vobj, parent, scope);
    state.build_children(vobj, scope);
    state
  }
}
