use gethostname::gethostname;
use std::env;

pub fn fetch() -> Vec<String> {
    #[cfg(target_os = "windows")]
    let username = env::var("USERNAME").unwrap();
    #[cfg(not(target_os = "windows"))]
    let username = env::var("USER").unwrap();
    vec![username, gethostname().into_string().unwrap()]
}
