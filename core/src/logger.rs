use directories_next::ProjectDirs;
use log::info;
use simplelog::*;
use std::fs::File;

pub fn logger() {
    let log_file = &ProjectDirs::from("rs", "", "brevium")
        .unwrap()
        .config_dir()
        .join("log.txt");

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_file).unwrap(),
        ),
    ])
    .unwrap_or_else(|_| {
        panic!(
            "Could not write to log file at {}",
            log_file.to_str().unwrap()
        )
    });

    info!("Logging to {}", log_file.to_str().unwrap());
}
