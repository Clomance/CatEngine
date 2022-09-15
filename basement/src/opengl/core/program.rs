#[cfg(target_os="windows")]
use crate::winapi::OpenGraphicsLibrary;

use super::{
    types::*,
    constants::*
};

use core::mem::transmute;

#[cfg(target_os="linux")]
extern "system"{
    fn glCreateProgram()->GLuint;
    fn glDeleteProgram(program:GLuint);
    fn glIsProgram(program:GLuint)->GLboolean;

    fn glAttachShader(program:GLuint,shader:GLuint);
    fn glDetachShader(program:GLuint,shader:GLuint);

    fn glLinkProgram(program:GLuint);

    fn glUseProgram(program:GLuint);

    fn glGetUniformBlockIndex(program:GLuint,uniform_block_name:*const GLchar)->GLuint;
    fn glUniformBlockBinding(program:GLuint,uniform_block_index:GLuint,uniform_block_binding:GLuint);

    fn glGetUniformLocation(program:GLuint,name:*const GLchar)->GLint;

    fn glGetProgramiv(program:GLuint,parameter:GLenum,value:*mut GLint);
    fn glGetProgramInfoLog(program:GLuint,max_length:GLsizei,length:*mut GLsizei,info_log:*mut GLchar);
}

#[cfg(target_os="windows")]
mod gl{
    pub static mut glCreateProgram:usize=0;
    pub static mut glDeleteProgram:usize=0;
    pub static mut glIsProgram:usize=0;

    pub static mut glAttachShader:usize=0;
    pub static mut glDetachShader:usize=0;

    pub static mut glLinkProgram:usize=0;

    pub static mut glUseProgram:usize=0;

    pub static mut glGetUniformLocation:usize=0;

    pub static mut glGetUniformBlockIndex:usize=0;
    pub static mut glUniformBlockBinding:usize=0;

    pub static mut glGetProgramiv:usize=0;
    pub static mut glGetProgramInfoLog:usize=0;
}

#[cfg(target_os="windows")]
mod gl_functions{
    use super::*;

    #[inline(always)]
    pub unsafe fn glCreateProgram()->GLuint{
        transmute::<usize,fn()->GLuint>(gl::glCreateProgram)()
    }
    #[inline(always)]
    pub unsafe fn glDeleteProgram(program:GLuint){
        transmute::<usize,fn(GLuint)>(gl::glDeleteProgram)(program)
    }
    #[inline(always)]
    pub unsafe fn glIsProgram(program:GLuint)->GLboolean{
        transmute::<usize,fn(GLuint)->GLboolean>(gl::glIsProgram)(program)
    }


    #[inline(always)]
    pub unsafe fn glAttachShader(program:GLuint,shader:GLuint){
        transmute::<usize,fn(GLuint,GLuint)>(gl::glAttachShader)(program,shader)
    }
    #[inline(always)]
    pub unsafe fn glDetachShader(program:GLuint,shader:GLuint){
        transmute::<usize,fn(GLuint,GLuint)>(gl::glDetachShader)(program,shader)
    }

    #[inline(always)]
    pub unsafe fn glLinkProgram(program:GLuint){
        transmute::<usize,fn(GLuint)>(gl::glLinkProgram)(program)
    }

    #[inline(always)]
    pub unsafe fn glUseProgram(program:GLuint){
        transmute::<usize,fn(GLuint)>(gl::glUseProgram)(program)
    }

    #[inline(always)]
    pub unsafe fn glGetUniformLocation(program:GLuint,name:*const GLchar)->GLint{
        transmute::<usize,fn(GLuint,*const GLchar)->GLint>(gl::glGetUniformLocation)(program,name)
    }

    #[inline(always)]
    pub unsafe fn glGetUniformBlockIndex(program:GLuint,uniform_block_name:*const GLchar)->GLuint{
        transmute::<usize,fn(GLuint,*const GLchar)->GLuint>(gl::glGetUniformBlockIndex)(program,uniform_block_name)
    }
    #[inline(always)]
    pub unsafe fn glUniformBlockBinding(program:GLuint,uniform_block_index:GLuint,uniform_block_binding:GLuint){
        transmute::<usize,fn(GLuint,GLuint,GLuint)>(gl::glUniformBlockBinding)(program,uniform_block_index,uniform_block_binding)
    }

    #[inline(always)]
    pub unsafe fn glGetProgramiv(program:GLuint,parameter:GLenum,values:*const GLint){
        transmute::<usize,fn(GLuint,GLenum,*const GLint)>(gl::glGetProgramiv)(program,parameter,values)
    }
    #[inline(always)]
    pub unsafe fn glGetProgramInfoLog(program:GLuint,max_length:GLsizei,length:*mut GLsizei,info_log:*mut GLchar){
        transmute::<usize,fn(GLuint,GLsizei,*mut GLsizei,*mut GLchar)>(gl::glGetProgramInfoLog)(
            program,
            max_length,
            length,
            info_log
        );
    }
}

#[cfg(target_os="windows")]
use gl_functions::*;

pub struct Program{
    // glGetUniformLocation:usize,
}

impl Program{
    #[cfg(target_os="windows")]
    pub fn load(library:&OpenGraphicsLibrary){
        unsafe{
            use gl::*;

            glCreateProgram=transmute(library.get_proc_address("glCreateProgram\0"));
            glDeleteProgram=transmute(library.get_proc_address("glDeleteProgram\0"));
            glIsProgram=transmute(library.get_proc_address("glIsProgram\0"));

            glAttachShader=transmute(library.get_proc_address("glAttachShader\0"));
            glDetachShader=transmute(library.get_proc_address("glDetachShader\0"));

            glLinkProgram=transmute(library.get_proc_address("glLinkProgram\0"));

            glUseProgram=transmute(library.get_proc_address("glUseProgram\0"));

            glGetUniformLocation=transmute(library.get_proc_address("glGetUniformLocation\0"));

            glGetUniformBlockIndex=transmute(library.get_proc_address("glGetUniformBlockIndex\0"));
            glUniformBlockBinding=transmute(library.get_proc_address("glUniformBlockBinding\0"));

            glGetProgramiv=transmute(library.get_proc_address("glGetProgramiv\0"));
            glGetProgramInfoLog=transmute(library.get_proc_address("glGetProgramInfoLog\0"));
        }
    }
}

impl Program{
    /// Creates a program object.
    /// 
    /// Creates an empty program object and returns a non-zero value by which it can be referenced.
    /// A program object is an object to which shader objects can be attached.
    /// This provides a mechanism to specify the shader objects that will be linked to create a program.
    /// It also provides a means for checking the compatibility of the shaders that will be used to create a program
    /// (for instance, checking the compatibility between a vertex shader and a fragment shader).
    /// When no longer needed as part of a program object, shader objects can be detached.
    /// 
    /// One or more executables are created in a program object by successfully attaching shader objects to it with `Program::attach_shader`,
    /// successfully compiling the shader objects with `Shader::compile`, and successfully linking the program object with `Program::link`.
    /// These executables are made part of current state when `Program::bind` is called.
    /// Program objects can be deleted by calling `Program::link`.
    /// The memory associated with the program object will be deleted when it is no longer part of current rendering state for any context.
    /// 
    /// Like buffer and texture objects, the name space for program objects may be shared across a set of contexts,
    /// as long as the server sides of the contexts share the same address space.
    /// If the name space is shared across contexts, any attached objects and the data associated with those attached objects are shared as well.
    /// 
    /// Applications are responsible for providing the synchronization across API calls when objects are accessed from different execution threads.
    /// 
    /// This function returns 0 if an error occurs creating the program object.
    #[inline(always)]
    pub unsafe fn create()->u32{
        glCreateProgram()
    }

    /// Deletes a program object.
    /// 
    /// `program` specifies the program object to be deleted.
    /// 
    /// Frees the memory and invalidates the name associated with the program object specified by `program`.
    /// This command effectively undoes the effects of a call to `Program::create`.
    /// 
    /// If a program object is in use as part of current rendering state, it will be flagged for deletion,
    /// but it will not be deleted until it is no longer part of current state for any rendering context.
    /// If a program object to be deleted has shader objects attached to it,
    /// those shader objects will be automatically detached but not deleted
    /// unless they have already been flagged for deletion by a previous call to `Shader::delete`.
    /// A value of 0 for `program` will be silently ignored.
    /// 
    /// To determine whether a program object has been flagged for deletion,
    /// call `Program::get` with arguments program and `ProgramParameter::DeleteStatus`.
    /// 
    /// `Error::InvalidValue` is generated if `program` is not a value generated by OpenGL.
    #[inline(always)]
    pub unsafe fn delete(program:u32){
        glDeleteProgram(program)
    }

    /// Determines if a name corresponds to a program object.
    /// 
    /// `program` specifies a potential program object.
    /// 
    /// Returns `true` if `program` is the name of a program object previously created with `Program::create`
    /// and not yet deleted with `Program::delete`.
    /// If `program` is zero or a non-zero value that is not the name of a program object,
    /// or if an error occurs, glIsProgram returns `false`.
    /// 
    /// No error is generated if `program` is not a valid program object name.
    /// 
    /// A program object marked for deletion with `Program::delete`
    /// but still in use as part of current rendering state is still considered a program object
    /// and `Program::is_program` will return `true`.
    #[inline(always)]
    pub unsafe fn is_program(program:u32)->bool{
        transmute(glIsProgram(program))
    }

}

impl Program{
    /// Attaches a shader object to a program object.
    /// 
    /// `program` specifies the program object to which a shader object will be attached.
    /// 
    /// `shader` specifies the shader object that is to be attached.
    /// 
    /// In order to create a complete shader program, there must be a way to specify the list of things that will be linked together.
    /// Program objects provide this mechanism.
    /// Shaders that are to be linked together in a program object must first be attached to that program object.
    /// `Program::attach_shader` attaches the shader object specified by `shader` to the program object specified by `program`.
    /// This indicates that `shader` will be included in link operations that will be performed on `program`.
    /// 
    /// All operations that can be performed on a shader object are valid whether or not the shader object is attached to a program object.
    /// It is permissible to attach a shader object to a program object before source code has been loaded into the shader object or before the shader object has been compiled.
    /// It is permissible to attach multiple shader objects of the same type because each may contain a portion of the complete shader.
    /// It is also permissible to attach a shader object to more than one program object.
    /// If a shader object is deleted while it is attached to a program object,
    /// it will be flagged for deletion, and deletion will not occur until `Program::detach_shader` is called to detach it from all program objects to which it is attached.
    /// 
    /// `Error::InvalidValue` is generated
    /// if either `program` or `shader` is not a value generated by OpenGL.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if `program` is not a program object,
    /// if `shader` is not a shader object,
    /// if `shader` is already attached to `program`.
    #[inline(always)]
    pub unsafe fn attach_shader(program:u32,shader:u32){
        glAttachShader(program,shader)
    }

    /// Detaches a shader object from a program object to which it is attached.
    /// 
    /// `program` specifies the program object from which to detach the shader object.
    /// 
    /// `shader` specifies the shader object to be detached.
    /// 
    /// Detaches the shader object specified by `shader` from the program object specified by `program`.
    /// This command can be used to undo the effect of the command `Program::attach_shader`.
    /// 
    /// If shader has already been flagged for deletion by a call to `Shader::delete`
    /// and it is not attached to any other program object, it will be deleted after it has been detached.
    /// 
    /// `Error::InvalidValue` is generated
    /// if either `program` or `shader` is not a value generated by OpenGL.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if `program` is not a program object,
    /// if `shader` is not a shader object,
    /// if `shader` is already attached to `program`.
    #[inline(always)]
    pub unsafe fn detach_shader(program:u32,shader:u32){
        glDetachShader(program,shader)
    }

    /// Links a program object.
    /// 
    /// `program` specifies the handle of the program object to be linked.
    /// 
    /// Links the program object specified by program.
    /// If any shader objects of type `ShaderType::VertexShader` are attached to `program`,
    /// they will be used to create an executable that will run on the programmable vertex processor.
    /// If any shader objects of type `ShaderType::GeometryShader` are attached to `program`,
    /// they will be used to create an executable that will run on the programmable geometry processor.
    /// If any shader objects of type `ShaderType::FragmentShader` are attached to `program`,
    /// they will be used to create an executable that will run on the programmable fragment processor.
    /// 
    /// The status of the link operation will be stored as part of the program object's state.
    /// This value will be set to `true` if the program object was linked without errors and is ready for use, and `false` otherwise.
    /// It can be queried by calling `Program::get` with arguments `program` and `ProgramParameter::LinkStatus`.
    /// 
    /// As a result of a successful link operation, all active user-defined uniform variables belonging to `program` will be initialized to 0,
    /// and each of the program object's active uniform variables will be assigned a location that can be queried by calling `Program::get_uniform_location`.
    /// Also, any active user-defined attribute variables that have not been bound to a generic vertex attribute index will be bound to one at this time.
    /// 
    /// Linking of a program object can fail for a number of reasons as specified in the OpenGL Shading Language Specification.
    /// The following lists some of the conditions that will cause a link error.
    /// - The number of active attribute variables supported by the implementation has been exceeded.
    /// - The storage limit for uniform variables has been exceeded.
    /// - The number of active uniform variables supported by the implementation has been exceeded.
    /// - The `main` function is missing for the vertex, geometry or fragment shader.
    /// - A varying variable actually used in the fragment shader is not declared in the same way (or is not declared at all) in the vertex shader, or geometry shader shader if present.
    /// - A reference to a function or variable name is unresolved.
    /// - A shared global is declared with two different types or two different initial values.
    /// - One or more of the attached shader objects has not been successfully compiled.
    /// - Binding a generic attribute matrix caused some rows of the matrix to fall outside the allowed maximum of `GL_MAX_VERTEX_ATTRIBS`.
    /// - Not enough contiguous vertex attribute slots could be found to bind attribute matrices.
    /// - The program object contains objects to form a fragment shader but does not contain objects to form a vertex shader.
    /// - The program object contains objects to form a geometry shader but does not contain objects to form a vertex shader.
    /// - The program object contains objects to form a geometry shader and the input primitive type, output primitive type, or maximum output vertex count is not specified in any compiled geometry shader object.
    /// - The program object contains objects to form a geometry shader and the input primitive type, output primitive type, or maximum output vertex count is specified differently in multiple geometry shader objects.
    /// - The number of active outputs in the fragment shader is greater than the value of `GL_MAX_DRAW_BUFFERS`.
    /// - The program has an active output assigned to a location greater than or equal to the value of `GL_MAX_DUAL_SOURCE_DRAW_BUFFERS` and has an active output assigned an index greater than or equal to one.
    /// - More than one varying out variable is bound to the same number and index.
    /// - The explicit binding assigments do not leave enough space for the linker to automatically assign a location for a `varying` out array, which requires multiple contiguous locations.
    /// - The `count` specified by `glTransformFeedbackVaryings` is non-zero, but the program object has no vertex or geometry shader.
    /// - Any variable name specified to `glTransformFeedbackVaryings` in the varyings array is not declared as an output in the vertex shader (or the geometry shader, if active).
    /// - Any two entries in the `varyings` array given `glTransformFeedbackVaryings` specify the same varying variable.
    /// - The total number of components to capture in any transform feedback varying variable is greater than the constant `GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS` and the buffer mode is `GL_SEPARATE_ATTRIBS`.
    /// 
    /// When a program object has been successfully linked, the program object can be made part of current state by calling `Program::bind`.
    /// Whether or not the link operation was successful, the program object's information log will be overwritten.
    /// The information log can be retrieved by calling `Program::get::info::log`.
    /// 
    /// `Program::link` will also install the generated executables as part of the current rendering state
    /// if the link operation was successful and the specified program object is already currently in use as a result of a previous call to `Program::bind`.
    /// If the program object currently in use is relinked unsuccessfully, its link status will be set to `false` ,
    /// but the executables and associated state will remain part of the current state until a subsequent call to `Program::bind` removes it from use.
    /// After it is removed from use, it cannot be made part of current state until it has been successfully relinked.
    /// 
    /// If `program` contains shader objects of type `ShaderType::VertexShader`, and optionally of type `ShaderType::GeometryShader`,
    /// but does not contain shader objects of type `ShaderType::FragmentShader`,
    /// the vertex shader executable will be installed on the programmable vertex processor,
    /// the geometry shader executable, if present, will be installed on the programmable geometry processor,
    /// but no executable will be installed on the fragment processor.
    /// The results of rasterizing primitives with such a program will be undefined.
    /// 
    /// The program object's information log is updated and the program is generated at the time of the link operation.
    /// After the link operation, applications are free to modify attached shader objects,
    /// compile attached shader objects, detach shader objects, delete shader objects, and attach additional shader objects.
    /// None of these operations affects the information log or the program that is part of the program object.
    /// 
    /// If the link operation is unsuccessful,any information about a previous link operation on `program` is lost
    /// (i.e., a failed link does not restore the old state of `program`).
    /// Certain information can still be retrieved from `program` even after an unsuccessful link operation.
    /// See for instance `glGetActiveAttrib` and `glGetActiveUniform`.
    /// 
    /// `Error::InvalidValue` is generated if `program` is not a value generated by OpenGL.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if `program` is not a program object,
    /// if `program` is the currently active program object and transform feedback mode is active.
    #[inline(always)]
    pub unsafe fn link(program:u32){
        glLinkProgram(program)
    }
}

impl Program{
    /// Installs a program object as part of current rendering state.
    /// 
    /// `program` specifies the handle of the program object whose executables are to be used as part of current rendering state.
    /// 
    /// Installs the program object specified by `program` as part of current rendering state.
    /// One or more executables are created in a program object by successfully attaching shader objects to it with `Program::attach_shader`,
    /// successfully compiling the shader objects with `Shader::compile`, and successfully linking the program object with `Program::link`.
    /// 
    /// A program object will contain an executable that will run on the vertex processor
    /// if it contains one or more shader objects of type `ShaderType::VertexShader` that have been successfully compiled and linked.
    /// A program object will contain an executable that will run on the geometry processor
    /// if it contains one or more shader objects of type `ShaderType::GeometryShader` that have been successfully compiled and linked.
    /// Similarly, a program object will contain an executable that will run on the fragment processor
    /// if it contains one or more shader objects of type `ShaderType::FragmentShader` that have been successfully compiled and linked.
    /// 
    /// While a program object is in use, applications are free to modify attached shader objects,
    /// compile attached shader objects, attach additional shader objects, and detach or delete shader objects.
    /// None of these operations will affect the executables that are part of the current state.
    /// However, relinking the program object that is currently in use will install the program object as part of the current rendering state
    /// if the link operation was successful (see `Program::link`).
    /// If the program object currently in use is relinked unsuccessfully, its link status will be set to `false`,
    /// but the executables and associated state will remain part of the current state until a subsequent call to `Program::bind` removes it from use.
    /// After it is removed from use, it cannot be made part of current state until it has been successfully relinked.
    /// 
    /// If `program` is zero, then the current rendering state refers to an `invalid` program object and the results of shader execution are undefined.
    /// However, this is not an error.
    /// 
    /// If `program` does not contain shader objects of type `ShaderType::FragmentShader`,
    /// an executable will be installed on the vertex, and possibly geometry processors,
    /// but the results of fragment shader execution will be undefined.
    /// 
    /// Like buffer and texture objects, the name space for program objects may be shared across a set of contexts,
    /// as long as the server sides of the contexts share the same address space.
    /// If the name space is shared across contexts, any attached objects and the data associated with those attached objects are shared as well.
    /// 
    /// Applications are responsible for providing the synchronization across API calls when objects are accessed from different execution threads.
    /// 
    /// `Error::InvalidValue` is generated if `program` is neither 0 nor a value generated by OpenGL.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if `program` is not a program object,
    /// if `program` could not be made part of current state,
    /// if transform feedback mode is active.
    #[inline(always)]
    pub unsafe fn bind(program:u32){
        glUseProgram(program)
    }
}

pub const InvalidIndex:u32=INVALID_INDEX;

impl Program{
    /// Returns the location of a uniform variable.
    /// 
    /// `program` specifies the program object to be queried.
    /// 
    /// `name` points to a null terminated string containing the name of the uniform variable whose location is to be queried.
    /// 
    /// Returns an integer that represents the location of a specific uniform variable within a program object.
    /// `name` must be a null terminated string that contains no white space.
    /// `name` must be an active uniform variable name in program that is not a structure, an array of structures, or a subcomponent of a vector or a matrix.
    /// This function returns -1 if name does not correspond to an active uniform variable in program or if name starts with the reserved prefix "gl_".
    /// 
    /// Uniform variables that are structures or arrays of structures may be queried by calling glGetUniformLocation for each field within the structure.
    /// The array element operator "[]" and the structure field operator "." may be used in name in order to select elements within an array or fields within a structure. The result of using these operators is not allowed to be another structure, an array of structures, or a subcomponent of a vector or a matrix. Except if the last part of name indicates a uniform variable array, the location of the first element of an array can be retrieved by using the name of the array, or by using the name appended by "[0]".
    /// 
    /// The actual locations assigned to uniform variables are not known until the program object is linked successfully.
    /// After linking has occurred, the command glGetUniformLocation can be used to obtain the location of a uniform variable.
    /// This location value can then be passed to glUniform to set the value of the uniform variable or to glGetUniform in order to query the current value of the uniform variable.
    /// After a program object has been linked successfully, the index values for uniform variables remain fixed until the next link command occurs.
    /// Uniform variable locations and values can only be queried after a link if the link was successful.
    /// 
    /// `Error::InvalidValue` is generated if `program` is not a value generated by OpenGL.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if `program` is not a program object,
    /// if `program` has not been successfully linked.
    #[inline(always)]
    pub unsafe fn get_uniform_location(program:u32,name:*const i8)->i32{
       glGetUniformLocation(program,name)
    }

    /// Retrieve the index of a named uniform block.
    /// 
    /// `program` specifies the name of a program containing the uniform block.
    /// 
    /// `name` specifies the address an array of characters to containing the name of the uniform block whose index to retrieve.
    /// 
    /// Retrieves the index of a uniform block within `program`.
    /// 
    /// `program` must be the name of a program object for which the command `Program::link` must have been called in the past,
    /// although it is not required that `Program::link` must have succeeded.
    /// The link could have failed because the number of active uniforms exceeded the limit.
    /// 
    /// `name` must contain a nul-terminated string specifying the name of the uniform block.
    /// 
    /// Returns the uniform block index for the uniform block named `name` of `program`.
    /// If `name` does not identify an active uniform block of `program`,
    /// returns the special identifier, `InvalidIndex`.
    /// Indices of the active uniform blocks of a program are assigned in consecutive order, beginning with zero.
    /// 
    /// `Error::InvalidOperation` is generated if `program` is not the name of a program object for which `Program::link` has been called in the past.
    /// 
    /// Available only if the GL version is 3.1 or greater.
    #[inline(always)]
    pub unsafe fn get_uniform_block_index(program:u32,uniform_block_name:*const i8)->u32{
        glGetUniformBlockIndex(program,uniform_block_name)
    }

    /// Assign a binding point to an active uniform block.
    /// 
    /// `program` is the name of a program object containing the active uniform block whose binding to assign.
    /// 
    /// `uniform_block_index` is the index of the active uniform block within program whose binding to assign.
    /// 
    /// `uniform_block_binding` specifies the binding point to which to bind the uniform block with index `uniform_block_index` within `program`.
    /// 
    /// Binding points for active uniform blocks are assigned using `Program::set_uniform_block_binding`.
    /// Each of a program's active uniform blocks has a corresponding uniform buffer binding point.
    /// `program` is the name of a program object for which the command glLinkProgram has been issued in the past.
    /// 
    /// If successful, `Program::set_uniform_block_binding` specifies that `program` will use the data store of the buffer object bound
    /// to the binding point `Program::set_uniform_block_binding` to extract the values of the uniforms in the uniform block identified by `uniform_block_index`.
    /// 
    /// When a program object is linked or re-linked, the uniform buffer object binding point assigned to each of its active uniform blocks is reset to zero.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `uniform_block_index` is not an active uniform block index of program,
    /// if `uniform_block_binding` is greater than or equal to the value of `MAX_UNIFORM_BUFFER_BINDINGS`,
    /// if `program` is not the name of a program object generated by the GL.
    /// 
    /// Available only if the GL version is 3.1 or greater.
    #[inline(always)]
    pub unsafe fn set_uniform_block_binding(program:u32,uniform_block_index:u32,uniform_block_binding:u32){
        glUniformBlockBinding(program,uniform_block_index,uniform_block_binding)
    }
}

#[repr(u32)]
#[derive(Clone,Copy,Debug)]
pub enum ProgramParameter{
    /// `values` returns `true` if `program` is currently flagged for deletion, and `false` otherwise.
    DeleteStatus=DELETE_STATUS,

    /// `values` returns `true` if the last link operation on `program` was successful, and `false` otherwise.
    LinkStatus=LINK_STATUS,

    /// `values` returns `true` or if the last validation operation on program was successful, and `false` otherwise.
    ValidateStatus=VALIDATE_STATUS,

    /// `values` returns the number of characters in the information log for `program` including the null termination character
    /// (i.e., the size of the character buffer required to store the information log).
    /// If `program` has no information log, a value of 0 is returned.
    InfoLogLength=INFO_LOG_LENGTH,

    /// `values` returns the number of shader objects attached to `program`.
    AttachedShaders=ATTACHED_SHADERS,

    /// `values` returns the number of active attribute variables for `program`.
    ActiveAttributes=ACTIVE_ATTRIBUTES,

    /// `values` returns the length of the longest active attribute name for program, including the null termination character
    /// (i.e., the size of the character buffer required to store the longest attribute name).
    /// If no active attributes exist, 0 is returned.
    ActiveAttributeMaxLength=ACTIVE_ATTRIBUTE_MAX_LENGTH,

    /// `values` returns the number of active uniform variables for `program`.
    ActiveUniforms=ACTIVE_UNIFORMS,

    /// `values` returns the length of the longest active uniform variable name for `program`,
    /// including the null termination character
    /// (i.e., the size of the character buffer required to store the longest uniform variable name).
    /// If no active uniform variables exist, 0 is returned.
    ActiveUniformMaxLength=ACTIVE_UNIFORM_MAX_LENGTH,

    /// Available only if the GL version 3.1 or greater.
    ActiveUniformBlocks=ACTIVE_UNIFORM_BLOCKS,

    /// Available only if the GL version 3.1 or greater.
    ActiveUniformBlockMaxNameLength=ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH,

    /// `values` returns a symbolic constant indicating the buffer mode used when transform feedback is active.
    /// This may be `GL_SEPARATE_ATTRIBS` or `GL_INTERLEAVED_ATTRIBS`.
    TransformFeedbackBufferMode=TRANSFORM_FEEDBACK_BUFFER_MODE,

    /// `values` returns the number of varying variables to capture in transform feedback mode for the program.
    TransformFeedbackVaryings=TRANSFORM_FEEDBACK_VARYINGS,

    /// `values` returns the length of the longest variable name to be used for transform feedback, including the null-terminator.
    TransformFeedbackVaryingMaxLength=TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH,

    /// `values` returns the maximum number of vertices that the geometry shader in `program` will output.
    /// 
    /// Accepted only if the GL version is 3.2 or greater.
    GeometryVerticesOut=GEOMETRY_VERTICES_OUT,

    /// `values` returns a symbolic constant indicating the primitive type accepted as input to the geometry shader contained in `program`.
    /// 
    /// Accepted only if the GL version is 3.2 or greater.
    GeometryInputType=GEOMETRY_INPUT_TYPE,

    /// `values` returns a symbolic constant indicating the primitive type that will be output by the geometry shader contained in `program`.
    /// 
    /// Accepted only if the GL version is 3.2 or greater.
    GeometryOutputType=GEOMETRY_OUTPUT_TYPE,
}

impl Program{
    /// Returns a parameter from a program object.
    /// 
    /// `program` specifies the program object to be queried.
    /// 
    /// `parameter` specifies the object parameter.
    /// 
    /// `value` returns the requested object parameter.
    /// 
    /// Returns in `value` the value of a parameter for a specific program object.
    /// 
    /// If an error is generated, no change is made to the contents of `values`.
    /// 
    /// `Error::InvalidValue` is generated if program is not a value generated by OpenGL.
    /// 
    /// `Error::InvalidOperation` is generated
    /// if `program` does not refer to a program object,
    /// if `parameter` is `ProgramParameter::GeometryVerticesOut`,
    /// `ProgramParameter::GeometryInputType`, or `ProgramParameter::GeometryOutputType`,
    /// and `program` does not contain a geometry shader.
    /// 
    /// `Error::InvalidEnum` is generated if `parameter` is not an accepted value.
    #[inline(always)]
    pub unsafe fn get_parameter(program:u32,parameter:ProgramParameter,values:*mut i32){
        glGetProgramiv(program,parameter as GLenum,values)
    }

    /// Returns the information log for a program object.
    /// 
    /// `program` specifies the program object whose information log is to be queried.
    /// 
    /// `max_length` specifies the size of the character buffer for storing the returned information log.
    /// 
    /// `length` returns the length of the string returned in `info_log` (excluding the null terminator).
    /// 
    /// `info_log` specifies an array of characters that is used to return the information log.
    /// 
    /// Returns the information log for the specified program object.
    /// The information log for a program object is modified when the program object is linked or validated.
    /// The string that is returned will be null terminated.
    /// 
    /// Returns in `info_log` as much of the information log as it can, up to a maximum of `max_length` characters.
    /// The number of characters actually returned, excluding the null termination character, is specified by `length`.
    /// If the length of the returned string is not required, a value of `NULL` can be passed in the `length` argument.
    /// The size of the buffer required to store the returned information log can be obtained
    /// by calling `Program::get` with the value `ProgramParameter::InfoLogLength`.
    /// 
    /// The information log for a program object is either an empty string,
    /// or a string containing information about the last link operation,
    /// or a string containing information about the last validation operation.
    /// It may contain diagnostic messages, warning messages, and other information.
    /// When a program object is created, its information log will be a string of length 0.
    /// 
    /// The information log for a program object is the OpenGL implementer's primary mechanism for conveying information about linking and validating.
    /// Therefore, the information log can be helpful to application developers during the development process, even when these operations are successful.
    /// Application developers should not expect different OpenGL implementations to produce identical information logs.
    /// 
    /// `Error::InvalidValue` is generated
    /// if `program` is not a value generated by OpenGL,
    /// if `max_length` is less than 0.
    /// 
    /// `Error::InvalidOperation` is generated if `program` is not a program object.
    #[inline(always)]
    pub unsafe fn get_info_log(program:u32,max_length:i32,length:*mut i32,info_log:*mut i8){
        glGetProgramInfoLog(program,max_length,length,info_log);
    }
}