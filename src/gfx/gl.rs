use glutin::prelude::GlDisplay;
use std::ffi::{CStr, CString};

include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

pub fn load(gl_context: &glutin::display::Display) {
    load_with(|symbol| gl_context.get_proc_address(&CString::new(symbol).unwrap()) as *const _);
    let version = unsafe {
        let data = CStr::from_ptr(GetString(VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };
    let mut vao = 0;
    unsafe {
        GenVertexArrays(1, &mut vao);
        BindVertexArray(vao);
    }
    println!("OpenGL version {}", version);
}
