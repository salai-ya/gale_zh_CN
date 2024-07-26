use anyhow::{anyhow, Context, Result};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use tauri::{AppHandle, Emitter, Manager};

use crate::util;
use serde::Serialize;
use std::{
    fs::{self, File},
    path::PathBuf,
};

#[derive(Serialize, Clone)]
struct JsError<'a> {
    name: &'a str,
    message: String,
}

pub fn log_js_err(name: &str, error: &anyhow::Error, handle: &AppHandle) {
    log::error!("{}: {:#}", name, error);
    handle
        .emit(
            "error",
            JsError {
                name,
                message: format!("{:#}", error),
            },
        )
        .ok();
}

fn log_path(app: &AppHandle) -> PathBuf {
    app.path().app_log_dir().unwrap().join("log.log")
}

pub fn setup(app: &AppHandle) -> Result<()> {
    let path = log_path(app);
    fs::create_dir_all(path.parent().unwrap()).context("failed to create log directory")?;
    let log_file = File::create(path).context("failed to create log file")?;

    let term_filter = match cfg!(debug_assertions) {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    CombinedLogger::init(vec![
        TermLogger::new(
            term_filter,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
    ])?;

    Ok(())
}

#[tauri::command]
pub fn open_gale_log(app: AppHandle) -> util::cmd::Result<()> {
    let path = log_path(&app);
    if !path.exists() {
        return Err(anyhow!("no log file found").into());
    }
    open::that(&path).context("failed to open log file")?;
    Ok(())
}

#[tauri::command]
pub fn log_err(msg: String) {
    log::error!("{}", msg);
}
