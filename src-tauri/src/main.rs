#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

#[derive(Clone)]
pub struct RobotState {
    pub position: Position,
    pub speed: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl RobotState {
    pub fn new() -> Self {
        Self {
            position: Position { x: 0.0, y: 0.0, z: 0.0 },
            speed: 1.0,
        }
    }
}

#[derive(Serialize)]
pub struct RobotStatus {
    position: Position,
    speed: f64,
}

pub struct AppState(pub Mutex<RobotState>);

#[tauri::command]
fn get_status(state: State<AppState>) -> RobotStatus {
    let robot = state.0.lock().unwrap();
    RobotStatus {
        position: robot.position.clone(),
        speed: robot.speed,
    }
}

#[tauri::command]
fn move_robot(x: f64, y: f64, z: f64, state: State<AppState>) -> RobotStatus {
    let mut robot = state.0.lock().unwrap();
    robot.position = Position { x, y, z };
    RobotStatus {
        position: robot.position.clone(),
        speed: robot.speed,
    }
}

#[tauri::command]
fn set_speed(speed: f64, state: State<AppState>) -> RobotStatus {
    let mut robot = state.0.lock().unwrap();
    robot.speed = speed;
    RobotStatus {
        position: robot.position.clone(),
        speed: robot.speed,
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState(Mutex::new(RobotState::new())))
        .invoke_handler(tauri::generate_handler![get_status, move_robot, set_speed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
