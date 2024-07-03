use simplelog::*;
use std::fs::{self, File};
use std::path::PathBuf;

pub(crate) fn init_log(verbose: u8, log: &Option<PathBuf>) {
    let level: LevelFilter = unsafe { std::mem::transmute((verbose + 1) as isize) };
    let console_config = ConfigBuilder::new()
        .set_level_padding(LevelPadding::Right)
        .set_target_level(LevelFilter::Error)
        .set_thread_level(LevelFilter::Error)
        .set_thread_mode(ThreadLogMode::Both)
        .set_time_level(LevelFilter::Error)
        .set_time_format_custom(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:7][offset_hour sign:mandatory]:[offset_minute]"))
        .set_time_offset_to_local()
        .unwrap()
        .build();
    let file_config = ConfigBuilder::new()
        .set_level_padding(LevelPadding::Right)
        .set_target_level(LevelFilter::Error)
        .set_thread_level(LevelFilter::Error)
        .set_thread_mode(ThreadLogMode::Both)
        .set_time_level(LevelFilter::Error)
        .set_time_format_custom(format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:7][offset_hour sign:mandatory]:[offset_minute]"))
        .set_time_offset_to_local()
        .unwrap()
        .build();

    let log_path = match log {
        Some(p) => {
            if !p.exists() {
                if let Some(d) = p.parent() {
                    println!("a");
                    fs::create_dir_all(d).ok();
                }
            }
            Some(p.clone())
        }
        _ => None,
    };

    let mut logs: Vec<Box<dyn SharedLogger>> = Vec::new();
    logs.push(TermLogger::new(
        level,
        console_config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ));
    if let Some(log_path) = log_path {
        logs.push(WriteLogger::new(
            level,
            file_config,
            File::create(log_path).unwrap(),
        ))
    }
    CombinedLogger::init(logs).unwrap();
}
