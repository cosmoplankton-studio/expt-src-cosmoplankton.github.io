extern crate wasm_bindgen;
use super::experiments;
use super::app_state;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as Gl;


#[wasm_bindgen]
pub struct AppClient {
    app_window: web_sys::Window,
    app_canvas: web_sys::HtmlCanvasElement,
    gl_ctx: Gl,
    gl_program: experiments::webgl::SimpleStaticProgram,
}

#[wasm_bindgen]
impl AppClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let (gl_ctx, app_window, app_canvas) = experiments::webgl::init_webgl_context().unwrap();

        Self {
            gl_program: experiments::webgl::SimpleStaticProgram::new(&gl_ctx),
            gl_ctx: gl_ctx,
            app_canvas: app_canvas,
            app_window: app_window,
        }
    }

    pub fn pause_update(&self, pause: bool) -> Result<(), JsValue> {
        app_state::pause_update(pause);
        Ok(())
    }

    pub fn update(&self, time: f32, height: f32, width: f32) -> Result<(), JsValue> {
        // update the static app state with the new values
        app_state::update_app_state(time, height, width);
        experiments::webgl::update_webgl_canvas_dimensions(&self.gl_ctx, &self.app_canvas, &self.app_window,)?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        // clear the color and depth buffer
        self.gl_ctx.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);
        // get the static app state
        let curr_app_state = app_state::get_app_state();
        // ask program to render
        self.gl_program.render(&self.gl_ctx, curr_app_state.time, curr_app_state.app_height, curr_app_state.app_width, curr_app_state.paused);
        Ok(())
    }
}
