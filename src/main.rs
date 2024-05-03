#[macro_use]
extern crate lazy_static;

mod body;
mod controller;
mod num;
mod apps;
mod vector;
mod config;


use crate::apps::main as real_main;
use crate::config::EResult;

#[cfg(feature = "color-eyre")]
fn init() -> color_eyre::Result<()> {
    color_eyre::install()?;
    Ok(())
}

fn main() -> EResult<()> {
    #[cfg(feature = "color-eyre")]
    init().unwrap();
    
    real_main()?;
    
    Ok(())
}