use config::RuaConfig;

pub(crate) mod config;
pub(crate) mod dart;

fn main() {
    let config = RuaConfig::load_or_default();
    dbg!(&config);
}
