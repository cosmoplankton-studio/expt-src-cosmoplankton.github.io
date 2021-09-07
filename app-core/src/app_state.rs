use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::*;

lazy_static! {
    static ref S_APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

/// Get the static app state
pub fn get_app_state() -> Arc<AppState> {
    S_APP_STATE.lock().unwrap().clone()
}

/// Update app state
pub fn update_app_state(time: f32, height: f32, width: f32) {
    let mut state = S_APP_STATE.lock().unwrap();
    *state = Arc::new(AppState {
        time: time,
        app_height: height,
        app_width: width,
        paused: state.paused,
    })
}

/// Pause or unpause the updates
pub fn pause_update(pause: bool) {
    let mut state = S_APP_STATE.lock().unwrap();
    *state = Arc::new(AppState {
        time: state.time,
        app_height: state.app_height,
        app_width: state.app_width,
        paused: pause,
    })
}

/// struct for the static app state
pub struct AppState {
    pub time: f32,
    pub app_height: f32,
    pub app_width: f32,
    pub paused: bool,
}

/// implementation of AppState
impl AppState {
    fn new() -> Self {
        Self {
            time: 0.,
            app_height: 0.,
            app_width: 0.,
            paused: false,
        }
    }
}
