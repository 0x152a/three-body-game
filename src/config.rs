use std::fs::File;
use std::io::Read;
use anyhow::Context;
use serde::Deserialize;
pub use anyhow::Result as EResult;
#[cfg(feature = "clap")]
use clap::Parser;

#[cfg(feature = "clap")]
fn default_config_file() -> String {
    // "examples/three_1.toml".into()
    "examples/round_4.toml".into()
}

#[cfg(feature = "clap")]
pub fn load_config<T>(filename: &str) -> EResult<T>
    where for<'de> T: Deserialize<'de> {
    let mut file = File::open(filename)
        .with_context(|| format!("Failed to open config file: {}", filename))
        ?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .with_context(|| format!("Failed to read config file: {}", filename))
        ?;
    let config: T = toml::from_str(content.as_str())
        .with_context(|| format!("Failed to parse config file: {}", filename))
        ?;
    Ok(config)
}

#[cfg(not(feature = "clap"))]
pub fn load_config<T>() -> EResult<T>
    where for<'de> T: Deserialize<'de> {
    let config: T = toml::from_str(include_str!("../examples/round_4.toml"))
        ?;
    Ok(config)
}

#[cfg(feature = "clap")]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = default_config_file())]
    file_name: String,
}

#[cfg(feature = "clap")]
pub fn init<T>() -> EResult<T>
    where for<'de> T: Deserialize<'de>
{
    let args = Args::parse();
    Ok(load_config::<T>(&args.file_name)?)
}

#[cfg(not(feature = "clap"))]
pub fn init<T>() -> EResult<T>
    where for<'de> T: Deserialize<'de>
{
    Ok(load_config::<T>()?)
}
