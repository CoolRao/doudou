mod core;

use std::fs;
use rustc_serialize::json;
use crate::core::PrjConfig;

fn main() -> std::io::Result<()> {
    let mut json_config = String::new();
    json_config = readConfig("config.json");
    println!("{}", json_config);
    let config: PrjConfig = json::decode(&*json_config).unwrap();
    core::run(config)
}

// 读取配置文件
fn readConfig(path: &str) -> String {
    let text = fs::read_to_string(path).unwrap();
    return text;
}

