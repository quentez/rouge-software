pub mod component;
pub mod scope;
pub mod vnode;
pub mod vstate;

use adw::{
  gio::{
    prelude::{ApplicationExt, ApplicationExtManual},
    Cancellable,
  },
  glib::{
    object::{Cast, ObjectExt},
    ExitCode,
  },
  Application,
};
use colored::Colorize;
use component::{Component, PartialComponentTask};
use log::debug;
use scope::Scope;
use std::{env, iter::once};

pub fn start<C: 'static + Component>() -> (Application, Scope<C>) {
  let partial_task = PartialComponentTask::<C, C>::new(Default::default(), None, None);

  let app: Application = partial_task.object().downcast().unwrap_or_else(|_| {
    panic!(
      "The top level object must be an Application, but {} was found.",
      partial_task.object().type_()
    )
  });
  app.set_default();
  app
    .register(None as Option<&Cancellable>)
    .expect("Unable to register Application.");

  let scope = partial_task.scope();
  let const_app = app.clone();

  let constructor = once(AndThen::DoNothing, move |_| {
    let (channel, task) = partial_task.finalise();
    MainContext::ref_thread_default().spawn_local(task);
    channel.unbounded_send(ComponentMessage::Mounted).unwrap();
    const_app.connect_shutdown(move |_| {
      channel.unbounded_send(ComponentMessage::Unmounted).unwrap();
    });
  });

  app.connect_activate(move |_| {
    debug!("{}", "Application has activated.".bright_blue());
    constructor(());
  });

  (app, scope)
}

pub fn run<C: 'static + Component>() -> ExitCode {
  let (app, _) = start::<C>();
  let args: Vec<String> = env::args().collect();
  app.run_with_args(&args)
}
