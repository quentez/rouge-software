use adw::{
  gio::{prelude::FileMonitorExt, Cancellable},
  glib::Error,
};
use libflatpak::{
  prelude::{InstallationExt, InstalledRefExt},
  Installation,
};

#[derive(Clone, Debug)]
pub struct FpRef {
  pub name: String,
  pub summary: String,
  pub version: String,
}

pub fn list() -> Result<Vec<FpRef>, Error> {
  let cancellable: Option<&Cancellable> = None;
  let installation = Installation::new_system(cancellable)?;
  let refs = installation.list_installed_refs(cancellable)?;

  let refs = refs
    .iter()
    .map(|f| FpRef {
      name: f.appdata_name().map_or("".to_string(), |s| s.to_string()),
      summary: f
        .appdata_summary()
        .map_or("".to_string(), |s| s.to_string()),
      version: f
        .appdata_version()
        .map_or("".to_string(), |s| s.to_string()),
    })
    .collect();

  Ok(refs)
}
