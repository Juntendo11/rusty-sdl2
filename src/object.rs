use std::{ffi::{CStr, CString}, ptr::{null,null_mut}};

use gl::{
    types::{GLchar, GLenum,GLint,GLuint},
    UseProgram,
};

pub struct Shader {
    id: GLint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind:GLenum) -> Result<Self, String>{
        let id: u32 = unsafe{ gl::CreateShader(type:kind)};
        unsafe{
            gl::ShaderSource(id,1,&source.as_ptr(), null());
            gl::CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe {gl::GetShaderiv(id,gl::COMPILE_STATUS, &mut success);}
        
        if success == 0 {
            //C compile error occurred
            let mut len: GLint = 0;
            unsafe{ gl::GetShaderiv(id,gl::INFO_LOG_LENGTH, & mut len);}
            let error: CString = create_whitespace_cstring_with_len(len as usize);

            unsafe {gl::GetShaderInfoLog(id, len,null_mut(),error.as_ptr() as *mut GLchar);}
            return Err(error.to_string_lossy().into_owned());
        }
        Ok(Shader{id})
    }

    pub fn id
}

fn create_whitespace_cstring_with_len(len: usize)-> CString {
    let mut buffer: Vec<u8> = Vec<u8>::with_capacity(len+1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe{Cstring::from_vec_unchecked(buffer)};
}



//OpenGL Program calls sequence of Shader calls
pub struct Program{
    id: GLuint,
}