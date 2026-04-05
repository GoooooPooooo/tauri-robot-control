#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

#[derive(Clone)]
pub struct RobotState {
    pub direction: Option<String>,
    pub speed: f64,
}

#[derive(Serialize)]
pub struct RobotStatus {
    direction: Option<String>,
    speed: f64,
}

pub struct AppState(pub Mutex<RobotState>);

#[tauri::command]
fn get_status(state: State<AppState>) -> RobotStatus {
    let robot = state.0.lock().unwrap();
    RobotStatus {
        direction: robot.direction.clone(),
        speed: robot.speed,
    }
}

#[tauri::command]
fn move_direction(direction: String, state: State<AppState>) -> RobotStatus {
    let mut robot = state.0.lock().unwrap();
    robot.direction = Some(direction);
    println!("Moving: {:?}", robot.direction);
    RobotStatus {
        direction: robot.direction.clone(),
        speed: robot.speed,
    }
}

#[tauri::command]
fn stop(state: State<AppState>) -> RobotStatus {
    let mut robot = state.0.lock().unwrap();
    robot.direction = None;
    println!("Stopped");
    RobotStatus {
        direction: robot.direction.clone(),
        speed: robot.speed,
    }
}

#[tauri::command]
fn set_speed(speed: f64, state: State<AppState>) -> RobotStatus {
    let mut robot = state.0.lock().unwrap();
    robot.speed = speed;
    println!("Speed set to: {}", speed);
    RobotStatus {
        direction: robot.direction.clone(),
        speed: robot.speed,
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState(Mutex::new(RobotState {
            direction: None,
            speed: 1.0,
        })))
        .invoke_handler(tauri::generate_handler![
            get_status,
            move_direction,
            stop,
            set_speed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
