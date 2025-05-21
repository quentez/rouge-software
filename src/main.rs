use adw::glib;
use rouge_software::{components::app::AppModel, reactive};
use std::process;

fn main() -> glib::ExitCode {
  reactive::run::<AppModel>()
}
