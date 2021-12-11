mod ones;
mod config;
mod message;
mod alert;

use clap::{App, Arg};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("ONES Manhour Record Alert")
        .version("0.1.2")
        .author("RustPanic <rustpanic@gmail.com>")
        .about("ONES 工时登记提醒机器人\nRepository: https://github.com/k8scat/ones-manhour-record-alert.git")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .default_value("./config.yml")
            .help("Sets a config file")
            .takes_value(true))
        .get_matches();

    let config_file = matches.value_of("config").unwrap();
    let mut conf = config::load_config(config_file).unwrap();
    conf.validate().unwrap();
    alert::alert(&conf).await.unwrap();
    Ok(())
}



