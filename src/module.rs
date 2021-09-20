pub mod host {
    pub fn fetch() -> Vec<String> {
        vec![whoami::username(), whoami::hostname()]
    }
}

pub mod os {
    pub fn fetch() -> String {
        whoami::distro()
    }
}

pub mod uptime {
    use sysinfo::{System, SystemExt, RefreshKind};

    pub fn fetch() -> String {
        let sys = System::new_with_specifics(RefreshKind::new());
        format!("{}", sys.uptime())
    }
}

pub mod memory {
    use sysinfo::{System, SystemExt, RefreshKind};

    pub fn fetch() -> String {
        let sys = System::new_with_specifics(RefreshKind::new().with_memory());
        format!("{}/{}", sys.used_memory(), sys.total_memory())
    }
}
