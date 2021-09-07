use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext as Gl;
use super::super::super::*; // [rust-standard-check] : check if this is appropriate


/// Initialize webgl context and setup some states for the webgl statemachine
/// Few code samples from [rustwasm example](https://rustwasm.github.io/docs/wasm-bindgen/examples/dom.html)
pub fn init_webgl_context() -> Result<(Gl, web_sys::Window, web_sys::HtmlCanvasElement,), JsValue> {
    let app_window = web_sys::window().expect("no global `window` exists");
    let app_document = app_window.document().expect("should have a document on window");
    let app_canvas = app_document.get_element_by_id("appCanvas").expect("document should have a canvas element with id - appCanvas");
    let app_canvas : web_sys::HtmlCanvasElement = app_canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl_context : Gl = app_canvas.get_context("webgl")?.unwrap().dyn_into()?; 

    gl_context.enable(Gl::BLEND);
    gl_context.blend_func(Gl::SRC_ALPHA, Gl::ONE_MINUS_SRC_ALPHA);
    gl_context.clear_color(0.1, 0.1, 0.1, 1.0);
    gl_context.clear_depth(1.);

    // return the tuple with all the details
    Ok((gl_context, app_window, app_canvas,))
}


/// Update the canvas width as per the window width
/// Resource for the below conversions: [JsValue doc](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html)
/// JavaScript does not have native integers. Instead it represents all numeric values with a single Number type which is
/// represented as an [IEEE 754 floating-point](https://en.wikipedia.org/wiki/IEEE_754) value.
/// If you need Js interoperable integers you could have a look at this crate (js_int)[https://docs.rs/js_int/0.2.0/js_int/]
pub fn update_webgl_canvas_dimensions(
    gl_context: &Gl,
    app_canvas: &web_sys::HtmlCanvasElement,
    _app_window: &web_sys::Window,
) -> Result<(), String> {
    /*
    * If our canvas is occupying the whole window this also works.
    let window_width = _app_window
        .inner_width()
        .expect("window width should be available")
        .as_f64()
        .expect("windoe width should be a number in Js") as u32;
    let window_height = _app_window
        .inner_height()
        .expect("window height should be available")
        .as_f64()
        .expect("windoe height should be a number in Js") as u32;
    if (window_height != app_canvas.height()) || (window_width != app_canvas.width()) {
        app_canvas.set_width(window_width);
        app_canvas.set_height(window_height);
        gl_context.viewport(0, 0, window_width as i32, window_height as i32);
    }
    */

    // Lookup the size the browser is displaying the canvas in CSS pixels as set it as canvas size.
    // [source](https://webglfundamentals.org/webgl/lessons/webgl-resizing-the-canvas.html)
    let display_width = app_canvas.client_width() as u32;
    let display_height = app_canvas.client_height() as u32;
    if (display_width != app_canvas.width()) || (display_height != app_canvas.height()) {
        app_canvas.set_width(display_width);
        app_canvas.set_height(display_height);

        gl_context.viewport(0, 0, display_width as i32, display_height as i32);
    }

    Ok(())
}


/// Create a webgl program
/// Code adapted from [rustwasm example](https://rustwasm.github.io/docs/wasm-bindgen/examples/webgl.html)
pub fn link_gl_program(
    gl_ctx: &Gl,
    vert_shader_src: &str,
    frag_shader_src: &str,
) -> Result<web_sys::WebGlProgram, String> {

    // log("link_gl_program() - 01");

    let program = gl_ctx.create_program()
                    .ok_or_else(|| String::from("Error : creation of gl program failed"))?;

    let vert_shader = compile_gl_shader(&gl_ctx, Gl::VERTEX_SHADER, &vert_shader_src)?;
    let frag_shader = compile_gl_shader(&gl_ctx, Gl::FRAGMENT_SHADER, &frag_shader_src)?;

    // log("link_gl_program() - 02");

    gl_ctx.attach_shader(&program, &vert_shader);
    gl_ctx.attach_shader(&program, &frag_shader);
    gl_ctx.link_program(&program);

    if gl_ctx
        .get_program_parameter(&program, Gl::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        log("link_gl_program() - error");
        Err(gl_ctx.get_program_info_log(&program).unwrap_or_else(|| String::from("Error: gl program linking failed")))
    }
}


/// Compile a webgl shader
/// Code adapted from [rustwasm example](https://rustwasm.github.io/docs/wasm-bindgen/examples/webgl.html)
pub fn compile_gl_shader(
    gl_ctx: &Gl,
    shader_type: u32,
    shader_src: &str,
) -> Result<web_sys::WebGlShader, String> {

    let shader = gl_ctx
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error: gl shader creation failed"))?;

    gl_ctx.shader_source(&shader, shader_src);
    gl_ctx.compile_shader(&shader);
    
    if gl_ctx
        .get_shader_parameter(&shader, Gl::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        gl_ctx.delete_shader(Some(&shader));
        Err(gl_ctx
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Error: gl shader compilation failed")))
    }
}
