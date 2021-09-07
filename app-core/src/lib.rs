extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
mod app_state;
mod app_client;
mod experiments;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
  log("client is loaded!");
}

#[wasm_bindgen]
pub fn pause_update(pause: bool) {
    let mut msg = "unpause";
    if pause {msg ="pause";}

    log(msg);

    let client = app_client::AppClient::new();
    let pause_log = client.pause_update(pause);
    if pause_log.is_err() { log("pause update error"); }
}

#[wasm_bindgen]
pub fn tick(time: f32, height: f32, width: f32) {
  // log("frame tick!");

  // log("client new!");
  let mut client = app_client::AppClient::new();

  let update_log = client.update(time, height, width);
  if update_log.is_err() { log("tick update error"); }

  let render_log = client.render();
  if render_log.is_err() { log("tick render error"); }
}
