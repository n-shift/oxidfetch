#[macro_use]
extern crate serde_derive;

use anyhow::Result;

mod config;
mod module;
mod render;
mod script;

fn main() -> Result<()> {
    if let Ok(cfg) = config::Config::fetch_msgpack() {
        render::display(cfg);
    } else {
        let cfg = script::extract_config()?;
        render::display(cfg.clone());
        cfg.cache()?;
    }

    Ok(())
}
