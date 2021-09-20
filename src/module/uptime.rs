use sysinfo::{SystemExt, System, RefreshKind};

pub fn fetch() -> String {
    let sys = System::new_with_specifics(RefreshKind::new());
    format!("{}", sys.uptime())
}
