#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::sync::Mutex;
use std::net::SocketAddr;
use axum::{
    extract::State,
    routing::get,
    Router,
};
use tauri::State as TauriState;

#[derive(Clone)]
pub struct RobotState {
    pub direction: Option<String>,
    pub speed: f64,
}

#[derive(Serialize, Clone)]
pub struct RobotStatus {
    direction: Option<String>,
    speed: f64,
}

pub struct HttpState {
    inner: Mutex<RobotState>,
}

impl Clone for HttpState {
    fn clone(&self) -> Self {
        HttpState {
            inner: Mutex::new(self.inner.lock().unwrap().clone()),
        }
    }
}

async fn get_command(State(state): State<HttpState>) -> &'static str {
    let robot = state.inner.lock().unwrap();
    match &robot.direction {
        Some(dir) => {
            if dir == "forward" {
                "FORWARD"
            } else if dir == "backward" {
                "BACKWARD"
            } else if dir == "left" {
                "LEFT"
            } else if dir == "right" {
                "RIGHT"
            } else {
                "STOP"
            }
        }
        None => "STOP",
    }
}

async fn get_status_http(State(state): State<HttpState>) -> axum::Json<RobotStatus> {
    let robot = state.inner.lock().unwrap();
    axum::Json(RobotStatus {
        direction: robot.direction.clone(),
        speed: robot.speed,
    })
}

async fn get_info() -> &'static str {
    "Robot Car Control Server\nESP32 connect to http://<PC_IP>:8080/command"
}

fn start_http_server(state: HttpState) {
    tokio::spawn(async move {
        let app = Router::new()
            .route("/command", get(get_command))
            .route("/status", get(get_status_http))
            .route("/", get(get_info))
            .with_state(state);
        
        let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
        println!("\n========================================");
        println!("  HTTP Server for ESP32");
        println!("========================================");
        println!("  ESP32 connect to: http://<YOUR_PC_IP>:8080/command");
        println!("  Or: http://localhost:8080/command");
        println!("========================================\n");
        
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
}

#[tauri::command]
fn get_status(state: TauriState<AppState>) -> RobotStatus {
    let robot = state.0.lock().unwrap();
    RobotStatus {
        direction: robot.direction.clone(),
        speed: robot.speed,
    }
}

#[tauri::command]
fn move_direction(direction: String, state: TauriState<AppState>) -> RobotStatus {
    let mut robot = state.0.lock().unwrap();
    robot.direction = Some(direction);
    println!("Moving: {:?}", robot.direction);
    RobotStatus {
        direction: robot.direction.clone(),
        speed: robot.speed,
    }
}

#[tauri::command]
fn stop(state: TauriState<AppState>) -> RobotStatus {
    let mut robot = state.0.lock().unwrap();
    robot.direction = None;
    println!("Stopped");
    RobotStatus {
        direction: robot.direction.clone(),
        speed: robot.speed,
    }
}

#[tauri::command]
fn set_speed(speed: f64, state: TauriState<AppState>) -> RobotStatus {
    let mut robot = state.0.lock().unwrap();
    robot.speed = speed;
    println!("Speed set to: {}", speed);
    RobotStatus {
        direction: robot.direction.clone(),
        speed: robot.speed,
    }
}

pub struct AppState(pub Mutex<RobotState>);

fn main() {
    let robot_state = RobotState { direction: None, speed: 1.0 };
    let app_state = AppState(Mutex::new(robot_state.clone()));
    let http_state = HttpState { inner: Mutex::new(robot_state) };
    
    start_http_server(http_state);
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![get_status, move_direction, stop, set_speed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
