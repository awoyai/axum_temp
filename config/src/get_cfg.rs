use super::cfg::Config;
use clap::Parser;
use once_cell::sync::Lazy;
use std::{fs::File, io::Read};

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, short, default_value = "./config.toml")]
    conf: String,
}

pub static CFG: Lazy<Config> = Lazy::new(self::Config::init);

impl Config {
    pub fn init() -> Self {
        let args: Args = Args::parse();
        println!("配置文件路径{}", &args.conf);
        let mut file = match File::open(&args.conf) {
            Ok(f) => f,
            Err(e) => panic!("不存在该文件：{}, 错误信息: {}", &args.conf, e),
        };
        let mut cfg_contens = String::new();
        match file.read_to_string(&mut cfg_contens) {
            Ok(s) => s,
            Err(e) => panic!("读取文件失败, 错误信息{}", e),
        };
        toml::from_str(&cfg_contens).expect("解析文件错误")
    }
}
