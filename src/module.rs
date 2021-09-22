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
    use super::convert_seconds;
    use sysinfo::{RefreshKind, System, SystemExt};

    pub fn fetch() -> String {
        let sys = System::new_with_specifics(RefreshKind::new());
        convert_seconds(sys.uptime() as f64)
    }
}

pub mod memory {
    use super::convert_kilobytes;
    use sysinfo::{RefreshKind, System, SystemExt};

    pub fn fetch() -> String {
        let sys = System::new_with_specifics(RefreshKind::new().with_memory());
        format!(
            "{}/{}",
            convert_kilobytes(sys.used_memory() as f64),
            convert_kilobytes(sys.total_memory() as f64)
        )
    }
}

use std::cmp;

pub fn convert_kilobytes(num: f64) -> String {
    let units = ["kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    if num < 1_f64 {
        return format!("{} {}", num, "kB");
    }
    let delimiter = 1000_f64;
    let exponent = cmp::min(
        (num.ln() / delimiter.ln()).floor() as i32,
        (units.len() - 1) as i32,
    );
    let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent))
        .parse::<f64>()
        .unwrap()
        * 1_f64;
    let unit = units[exponent as usize];
    format!("{} {}", pretty_bytes, unit)
}

pub fn convert_seconds(input: f64) -> String {
    const MIN: f64 = 60.;
    const HOUR: f64 = MIN * 60.;
    const DAY: f64 = HOUR * 24.;
    let days = (input / DAY).floor();
    let hours = ((input % DAY) / HOUR).floor();
    let mins = (((input % DAY) % HOUR) / MIN).floor();

    format!(
        "{}{}{}",
        {
            if days == 0. {
                String::new()
            } else {
                format!("{}d ", days)
            }
        },
        {
            if hours == 0. {
                String::new()
            } else {
                format!("{}h ", hours)
            }
        },
        {
            if mins == 0. {
                String::new()
            } else {
                format!("{}m ", mins)
            }
        }
    )
}
