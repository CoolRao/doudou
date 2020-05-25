
mod core;
use std::fs;
use rustc_serialize::json;
use crate::core::PrjConfig;
use log::info;

fn main() -> std::io::Result<()> {
    let json_config: String = read_config("config.json");
    println!("{}", json_config);
    let config: PrjConfig = json::decode(&*json_config).unwrap();
    init_logger();
    info!("application is running  ");
    core::run(config)
}

// read config json file
fn read_config(path: &str) -> String {
    let text = fs::read_to_string(path).unwrap();
    return text;
}

// init logger
fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),











                
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}
