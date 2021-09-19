#[macro_use]
extern crate serde_derive;

use anyhow::Result;

mod config;
mod render;
mod script;

fn main() -> Result<()> {
    let cfg = script::extract_config()?;
    render::display(cfg);

    Ok(())
}
