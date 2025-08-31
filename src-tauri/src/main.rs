// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;

fn main() -> Result<()> {
    fight_macros_lib::run().expect("Error al iniciar la aplicación");
    Ok(())
}
