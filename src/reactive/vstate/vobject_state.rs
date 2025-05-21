use adw::{
  gio::{
    prelude::{ActionExt, ActionMapExt},
    Action, Menu, MenuItem,
  },
  glib::{
    object::{Cast, IsA, ObjectExt},
    Object, SignalHandlerId,
  },
  prelude::{AdwWindowExt, BinExt},
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

fn add_child(parent: &Object, index: usize, total: usize, child: &Object) {
  // Application.
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
  }
  // // ApplicationWindow.
  // else if let Some(window) = parent.downcast_ref::<ApplicationWindow>() {
  //   // ApplicationWindow: takes any number of Actions, optionally one
  //   // ShortcutsWindow added with `set_help_overlay()`, and either 1 or 2
  //   // Widgets. If 1, it's the main widget. If 2, the first is added with
  //   // `set_titlebar()` and the second is the main widget.
  //   if let Some(action) = child.downcast_ref::<Action>() {
  //     window.add_action(action);
  //   } else if let Some(help_overlay) = child.downcast_ref::<ShortcutsWindow>() {
  //     window.set_help_overlay(Some(help_overlay));
  //   } else if let Some(widget) = child.downcast_ref::<Widget>() {
  //     window.set_child(Some(widget));
  //     // match window.child() {
  //     //   None => window.set_child(Some(widget)),
  //     //   Some(ref titlebar) if window.titlebar().is_none() => {
  //     //     window.set_titlebar(Some(titlebar));
  //     //     window.set_child(Some(widget));
  //     //   }
  //     //   _ => panic!("ApplicationWindow can have at most two Widget children."),
  //     // }
  //   } else {
  //     panic!(
  //       "ApplicationWindow's children must be Actions or Widgets, but {} was found.",
  //       child.type_()
  //     );
  //   }
  // }
  // Window.
  else if let Some(window) = parent.downcast_ref::<Window>() {
    // Window: takes only 1 or 2 Widgets. If 1 widget child, it's the
    // window's main widget. If 2, the first is the title bar and the second
    // is the main widget. More than 2 goes boom.
    if let Some(widget) = child.downcast_ref::<Widget>() {
      if total == 2 && index == 0 {
        window.set_titlebar(Some(widget));
      } else {
        window.set_content(Some(widget));
      }
    } else {
      panic!(
        "Window's children must be Widgets, but {} was found.",
        child.type_()
      );
    }
  }
  // HeaderBar.
  else if let Some(parent) = parent.downcast_ref::<HeaderBar>() {
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
  }
  // Box.
  else if let Some(parent) = parent.downcast_ref::<Box>() {
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
  }
  // Grid.
  else if let Some(parent) = parent.downcast_ref::<Grid>() {
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
  }
  // Other.
  else {
    panic!("Don't know how to add children to a {}", parent.type_());
  }
}

fn remove_child(parent: &Object, child: &Object) {
  // Application.
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
  }
  // ApplicationWindow.
  // else if let Some(window) = parent.downcast_ref::<ApplicationWindow>() {
  //   if let Some(action) = child.downcast_ref::<Action>() {
  //     window.remove_action(&action.name());
  //   } else if let Some(help_overlay) = child.downcast_ref::<ShortcutsWindow>() {
  //     window.set_help_overlay(None);
  //   } else if let Some(widget) = child.downcast_ref::<Widget>() {
  //     if window.titlebar().map_or(false, |w| w.eq(widget)) {
  //       window.set_titlebar(Option::<&Widget>::None);
  //     }

  //     if window.child().map_or(false, |w| w.eq(widget)) {
  //       window.set_child(Option::<&Widget>::None);
  //     }
  //   } else {
  //     panic!(
  //       "ApplicationWindow's children must be Actions or Widgets, but {} was found.",
  //       child.type_()
  //     );
  //   }
  // }
  // Window.
  else if let Some(window) = parent.downcast_ref::<Window>() {
    if let Some(widget) = child.downcast_ref::<Widget>() {
      if window.titlebar().is_some_and(|w| w.eq(widget)) {
        window.set_titlebar(Option::<&Widget>::None);
      }

      if window.content().is_some_and(|w| w.eq(widget)) {
        window.set_content(Option::<&Widget>::None);
      }
    } else {
      panic!(
        "Window's children must be Widgets, but {} was found.",
        child.type_()
      );
    }
  }
  // Box.
  else if let Some(parent) = parent.downcast_ref::<Box>() {
    if let Some(widget) = child.downcast_ref::<Widget>() {
      parent.remove(widget);
    } else {
      panic!(
        "Box's children must be Widgets, but {} was found.",
        child.type_()
      );
    }
  }
  // Grid.
  else if let Some(parent) = parent.downcast_ref::<Grid>() {
    if let Some(widget) = child.downcast_ref::<Widget>() {
      parent.remove(widget);
    } else {
      panic!(
        "Grid's children must be Widgets, but {} was found.",
        child.type_()
      );
    }
  }
  // Other.
  else {
    panic!("Don't know how to remove a child from a {}", parent.type_());
  }
}

impl<C: 'static + Component> VObjectState<C> {
  pub fn build_root(vobj: &VObject<C>, parent: Option<&Object>, scope: &Scope<C>) -> Self {
    // Build this object
    let object: Object = build_obj(&vobj);

    // // Apply properties
    // for prop in &vobj.properties {
    //   (prop.set)(object.upcast_ref(), parent, true);
    // }

    (vobj.patcher)(&object);

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
    let total_children = vobj.children.len();
    for (index, child_spec) in vobj.children.iter().enumerate() {
      let child = VState::build(child_spec, Some(&object), &scope);
      let child_object = child.object().clone();
      add_child(&object, index, total_children, &child_object);
      self.children.push(child);
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

  pub fn patch(&mut self, vobj: &VObject<C>, parent: Option<&Object>, scope: &Scope<C>) -> bool {
    // Patch children
    let mut to_remove = None;
    let mut to_append = Vec::new();
    let mut reconstruct_from = None;
    for index in 0..(self.children.len().max(vobj.children.len())) {
      match (self.children.get_mut(index), vobj.children.get(index)) {
        (Some(VState::Component(target)), Some(spec_item)) => {
          match spec_item {
            VNode::Object(_) => {
              // Component has become a widget; reconstruct from here
              reconstruct_from = Some(index);
              break;
            }
            VNode::Component(spec) => {
              if !target.patch(spec, Some(&self.object), scope) {
                reconstruct_from = Some(index);
                break;
              }
            }
          }
        }
        (Some(VState::Object(target)), Some(spec_item)) => {
          match spec_item {
            VNode::Object(spec) => {
              if target.object.type_() == spec.object_type {
                // Objects have same type; patch down
                target.patch(spec, Some(&self.object), scope);
              } else {
                // Objects are different, need to reconstruct everything from here
                reconstruct_from = Some(index);
                break;
              }
            }
            VNode::Component(_) => {
              // Gtk object has turned into a component; reconstruct from here
              reconstruct_from = Some(index);
              break;
            }
          }
        }
        (Some(_), None) => {
          // Extraneous Gtk object; delete
          if to_remove.is_none() {
            to_remove = Some(index);
          }
          break;
        }
        (None, Some(spec)) => {
          // New spec; construct
          let state = VState::build(spec, Some(&self.object), scope);
          add_child(&self.object, index, vobj.children.len(), state.object());
          to_append.push(state);
        }
        (None, None) => break,
      }
    }
    if let Some(index) = reconstruct_from {
      // Remove all previous children from here onwards
      if self.object.is::<Window>() && index == 0 && self.children.len() == 2 {
        panic!("Can't remove a title bar widget from an existing Window!");
      }
      for child in self.children.drain(index..) {
        remove_child(&self.object, child.object());
        child.unmount();
      }
      // Rebuild children from new specs
      for (index, child_spec) in vobj.children.iter().enumerate().skip(index) {
        let state = VState::build(child_spec, Some(&self.object), scope);
        add_child(&self.object, index, vobj.children.len(), state.object());
        if let Some(w) = state.widget() {
          w.set_visible(true);
        }
        self.children.push(state);
      }
    } else {
      // Remove children flagged as extraneous
      if let Some(remove_from) = to_remove {
        if self.object.is::<Window>() && remove_from == 1 && self.children.len() == 2 {
          panic!("Can't remove a title bar widget from an existing Window!");
        }
        for child in self.children.drain(remove_from..) {
          remove_child(&self.object, &child.object());
          child.unmount();
        }
      }
      // Or append newly constructed children
      if self.object.is::<Window>() && !to_append.is_empty() && self.children.len() == 1 {
        panic!("Can't add a title bar widget to an existing Window!");
      }
      for child in to_append {
        if let Some(w) = child.widget() {
          w.set_visible(true);
        }
        self.children.push(child);
      }
    }

    (vobj.patcher)(&self.object);

    // // Patch properties
    // self.patch_properties(&vobj.properties, parent);

    // // Patch child properties
    // self.patch_properties(&vobj.child_props, parent);

    // // Patch handlers
    // self.patch_handlers(&vobj.handlers, scope);

    true
  }

  pub fn unmount(self) {
    for child in self.children {
      child.unmount();
    }
  }
}
