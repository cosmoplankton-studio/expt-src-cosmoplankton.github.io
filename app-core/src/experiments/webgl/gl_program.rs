use super::super::common_utils as utils;
use super::gl_utils as gl_utils;
use super::super::super::*;
use web_sys::WebGlRenderingContext as Gl;
use web_sys::*;
use nalgebra::{Perspective3, Matrix4, Vector3};


pub struct SimpleStaticProgram {
    time_last_frame: f32,
    program: WebGlProgram,
    color_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    index_count: i32,
    position_buffer: WebGlBuffer,
    u_opacity: WebGlUniformLocation,
    u_projection: WebGlUniformLocation,
    a_position: u32,
    a_color: u32,
}

impl SimpleStaticProgram {
    pub fn new(gl: &Gl) -> Self {

        // log("SimpleStaticProgram::new() - 01");

        let program = gl_utils::link_gl_program(
            &gl,
            &super::gl_shader_vertex::SHADER,
            &super::gl_shader_fragment::SHADER,
        ).unwrap();

        // log("SimpleStaticProgram::new() - 02");

        // create the vertex buffers for the graphics
        let vertices_rect: [f32; 8] = [
            -1., 1.,
            -1., -1.,
            1., 1.,
            1., -1.,
        ];
        let buff_rect_vert = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&buff_rect_vert));

        // log("SimpleStaticProgram::new() - 03");
        
        // index buffer for indexed draw
        let indices_rect: [u16; 6] = [0, 1, 2, 2, 1, 3];
        let buff_rect_index = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, Some(&buff_rect_index));

        /*
        // At this point only aware of these two unsafe code versions to achieve this.
        // Is there a safe way to do this?
        // unsafe code 1 :
        use js_sys::WebAssembly;
        use wasm_bindgen::JsCast;
        let mem_buff_vert = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let vert_location = vertices_rect.as_ptr() as u32 / 4; // wasm_bindgen default [u8] 
        let vert_array = js_sys::Float32Array::new(&mem_buff_vert).subarray(
            vert_location,
            vert_location + vertices_rect.len() as u32,
        );
        gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &vert_array, Gl::STATIC_DRAW,);
        let mem_buff_index = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let index_location = indices_rect.as_ptr() as u32 / 2; // wasm_bindgen default [u8] 
        let index_array = js_sys::Uint16Array::new(&mem_buff_index).subarray(
            index_location,
            index_location + indices_rect.len() as u32,
        );
        gl.buffer_data_with_array_buffer_view(Gl::ELEMENT_ARRAY_BUFFER, &index_array, Gl::STATIC_DRAW,);
        */

        /*
        // unsafe code 2 [code-source](https://rustwasm.github.io/docs/wasm-bindgen/examples/webgl.html):
        // `Float32Array::view` is creating a raw view into wasm module's `WebAssembly.Memory` buffer,
        // after `Float32Array::view` we have to be very careful not to do any memory allocations before it's dropped.
        */
        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices_rect);
            gl.buffer_data_with_array_buffer_view(
                Gl::ARRAY_BUFFER,
                &vert_array,
                Gl::STATIC_DRAW,
            );

            let index_array = js_sys::Uint16Array::view(&indices_rect);
            gl.buffer_data_with_array_buffer_view(
                Gl::ELEMENT_ARRAY_BUFFER,
                &index_array,
                Gl::STATIC_DRAW,
            );
        }

        // color buffer for the gradient
        let buff_rect_color = gl.create_buffer().ok_or("failed to create color buffer").unwrap();
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&buff_rect_color));

        // color buffer for the gradients
        let colors_rect: [f32; 16] = [
            1., 0., 0., 1.,
            0., 1., 0., 1., 
            0., 0., 1., 1.,
            1., 1., 1., 1.,
        ];
        unsafe {
            let color_array = js_sys::Float32Array::view(&colors_rect);
            gl.buffer_data_with_array_buffer_view(Gl::ARRAY_BUFFER, &color_array, Gl::STATIC_DRAW);
        }

        // log("SimpleStaticProgram::new() - 04");

        let u_opacity = gl.get_uniform_location(&program, "uOpacity").unwrap();
        let u_projection = gl.get_uniform_location(&program, "uProjection").unwrap();
        let a_position = gl.get_attrib_location(&program, "aPosition") as u32;
        let a_color = gl.get_attrib_location(&program, "aColor") as u32;
        let index_count = indices_rect.len() as i32;

        // log("SimpleStaticProgram::new() - 05");

        Self {
            time_last_frame: 0.,
            u_opacity: u_opacity,
            u_projection: u_projection,
            program: program,
            color_buffer: buff_rect_color,
            index_buffer: buff_rect_index,
            index_count: index_count,
            position_buffer: buff_rect_vert,
            a_color: a_color,
            a_position: a_position,
        }
    }

    pub fn render(
        &mut self,
        gl: &Gl,
        time: f32,
        client_height: f32,
        client_width: f32,
        update_paused: bool,
    ) {
        // tick timer
        // let now = time; // nice effect!!!
        let now = 0.001 * time; // convert to second
        let then = self.time_last_frame;
        let mut delta = now - then;
        if update_paused { delta = 0.; }
        self.time_last_frame = now;


        // clear the scene
        gl.clear(Gl::COLOR_BUFFER_BIT  | Gl::DEPTH_BUFFER_BIT); // clear color and depth buffer

        // setup render program and buffers
        gl.use_program(Some(&self.program));

        let normalize = false;
        let stride = 0; // 0 = use attr_elem_type_* and elems_per_vertex_* below
        let offset = 0; // how many bytes inside the buffer to start from
        // vertex buffer
        let elems_per_vertex_pos = 2;  // number of elements to pull from the vertex-position-buffer per vertex
        let attr_elem_type_pos = Gl::FLOAT; // type of data in the vertex position attribute
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&self.position_buffer));
        gl.vertex_attrib_pointer_with_i32(self.a_position, elems_per_vertex_pos, attr_elem_type_pos, normalize, stride, offset);
        gl.enable_vertex_attrib_array(self.a_position);

        // color buffer
        let elems_per_vertex_color = 4;  // number of elements to pull from the vertex-color-buffer per vertex
        let attr_elem_type_color = Gl::FLOAT; // type of data in the vertex position attribute
        gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&self.color_buffer));
        gl.vertex_attrib_pointer_with_i32(self.a_color, elems_per_vertex_color, attr_elem_type_color, normalize, stride, offset);
        gl.enable_vertex_attrib_array(self.a_color);

        // setup the uniforms for opacity, projection-matrix, and the model-view-matrix
        // projection maatrix
        let fov = std::f32::consts::PI / 4.0;
        let aspect_ratio = client_width / client_height;
        let z_near = 0.1;
        let z_far = 100.;
        let projection_mat = Perspective3::new(aspect_ratio, fov, z_near, z_far).into_inner();
        // model-view matrix
        // let modelview_mat = Matrix4::<f32>::identity(); // identity model-view no rotation, same as 0.0 rotations
        let x_rot = delta; // let x_rot = 0.0;
        let y_rot = delta; // let y_rot = 0.0; // std::f32::consts::PI / 4.0;
        let z_rot = delta;
        let rotation_vec = Vector3::<f32>::new(x_rot, y_rot, z_rot);
        let modelview_mat = Matrix4::<f32>::new_rotation(rotation_vec); // rotation model-view
        let translate_matrix = utils::matrix_translation(0., 0., -10.);
        let scale_matrix = utils::matrix_scaling(1., 1., 1.);
        let transform_mat = utils::matrix_multiply(translate_matrix, modelview_mat);
        let transform_mat = utils::matrix_multiply(scale_matrix, transform_mat);
        let transform_mat = utils::matrix_multiply(projection_mat, transform_mat);
        gl.uniform1f(Some(&self.u_opacity), 1.0);
        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_projection), false, transform_mat.as_slice());

        // indexed draw
        let index_offset = 0;
        gl.bind_buffer(Gl::ELEMENT_ARRAY_BUFFER, Some(&self.index_buffer));
        gl.draw_elements_with_i32(Gl::TRIANGLES, self.index_count, Gl::UNSIGNED_SHORT, index_offset);
    }
}
