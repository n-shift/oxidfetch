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
    use sysinfo::{System, SystemExt, RefreshKind};

    pub fn fetch() -> String {
        let sys = System::new_with_specifics(RefreshKind::new());
        format!("{}", convert_seconds(sys.uptime() as f64))
    }
}

pub mod memory {
    use super::convert_kilobytes;
    use sysinfo::{System, SystemExt, RefreshKind};

    pub fn fetch() -> String {
        let sys = System::new_with_specifics(RefreshKind::new().with_memory());
        format!("{}/{}", convert_kilobytes(sys.used_memory() as f64), convert_kilobytes(sys.total_memory() as f64))
    }
}

use std::cmp;

pub fn convert_kilobytes(num: f64) -> String {
  let units = ["kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
  if num < 1_f64 {
      return format!("{} {}", num, "kB");
  }
  let delimiter = 1000_f64;
  let exponent = cmp::min((num.ln() / delimiter.ln()).floor() as i32, (units.len() - 1) as i32);
  let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent)).parse::<f64>().unwrap() * 1_f64;
  let unit = units[exponent as usize];
  format!("{} {}", pretty_bytes, unit)
}
// https://stackoverflow.com/a/8273826
pub fn convert_seconds(num: f64) -> String {
    let days = (num / (60 * 60 * 24) as f64).floor();
    let hours = ((num % (60 * 60 * 24) as f64) / (60 * 60) as f64).floor();
    let minutes = (((num % (60 * 60 * 24) as f64) % (60 * 60) as f64) / (60 * 60) as f64).floor();

    format!("{}d {}h {}m", days, hours, minutes)
}
