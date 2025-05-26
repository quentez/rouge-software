use adw::glib;
use rouge_software::{components::app::App, reactive};
use std::process;

fn main() -> glib::ExitCode {
  reactive::run::<App>()
}
