use adw::glib::object::IsA;
use adw::prelude::WidgetExt;
use gtk4::{Align, SizeGroup, Widget};

pub trait ReactiveWidgetExt {
  /// Attach widget to a `gtk::SizeGroup`.
  fn set_size_group(&self, size_group: &SizeGroup);

  /// Set margin at start, end, top and bottom all at once.
  fn set_margin_all(&self, margin: i32) {
    self.set_margin_horizontal(margin);
    self.set_margin_vertical(margin);
  }

  /// Set margin at top and bottom at once.
  fn set_margin_vertical(&self, margin: i32);

  /// Set margin at start and end at once.
  fn set_margin_horizontal(&self, margin: i32);

  /// Set both horizontal and vertical expand properties at once.
  fn set_expand(&self, expand: bool);

  /// Set both horizontal and vertical align properties at once.
  fn set_align(&self, align: Align);

  /// Add class name if active is [`true`] and
  /// remove class name if active is [`false`]
  fn set_class_active(&self, class: &str, active: bool);

  /// Sets the tooltip text of a widget and enables is.
  ///
  /// This is basically, the same as using [`WidgetExt::set_has_tooltip()`]
  /// and [`WidgetExt::set_tooltip_text()`], but with fewer steps.
  fn set_tooltip(&self, test: &str);
}

impl<T: IsA<Widget>> ReactiveWidgetExt for T {
  fn set_size_group(&self, size_group: &SizeGroup) {
    size_group.add_widget(self);
  }

  fn set_margin_vertical(&self, margin: i32) {
    self.set_margin_top(margin);
    self.set_margin_bottom(margin);
  }

  fn set_margin_horizontal(&self, margin: i32) {
    self.set_margin_start(margin);
    self.set_margin_end(margin);
  }

  fn set_class_active(&self, class: &str, active: bool) {
    if active {
      self.add_css_class(class);
    } else {
      self.remove_css_class(class);
    }
  }

  fn set_expand(&self, expand: bool) {
    self.set_hexpand(expand);
    self.set_vexpand(expand);
  }

  fn set_align(&self, align: Align) {
    self.set_halign(align);
    self.set_valign(align);
  }

  fn set_tooltip(&self, text: &str) {
    self.set_has_tooltip(true);
    self.set_tooltip_text(Some(text));
  }
}
