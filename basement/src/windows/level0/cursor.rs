use crate::windows::{
    WinCore,
    core::InstanceHandle,
    core::cursor::{
        CursorInfo,
        CursorHandle,
        SystemCursor,
        SystemCursorId,
    },
};

use super::error::Error;

use core::{
    mem::{
        transmute,
        transmute_copy,
        size_of,
    },
    ptr::{
        null_mut,
        NonNull
    },
};


/// Before closing, an application must call the `Cursor::destroy` function
/// to free any system resources associated with the cursor.
#[repr(transparent)]
pub struct Cursor{
    handle:CursorHandle,
}

impl Cursor{
    /// Creates a cursor having the specified size, bit patterns, and hot spot.
    /// 
    /// `and_plane` - An array of bytes that contains the bit values for the AND mask of the cursor,
    /// as in a device-dependent monochrome bitmap.
    /// 
    /// `xor_plane` - An array of bytes that contains the bit values for the XOR mask of the cursor,
    /// as in a device-dependent monochrome bitmap.
    /// 
    /// The `width` and `height` parameters must specify a width and height
    /// that are supported by the current display driver,
    /// because the system cannot create cursors of other sizes.
    /// To determine the width and height supported by the display driver,
    /// use the `GetSystemMetrics` function, specifying the `SM_CXCURSOR` or `SM_CYCURSOR` value.
    /// 
    /// This API does not participate in DPI virtualization.
    /// The output returned is in terms of physical coordinates,
    /// and is not affected by the DPI of the calling thread.
    /// Note that the cursor created may still be scaled
    /// to match the DPI of any given window it is drawn into.
    #[inline(always)]
    pub fn create(
        &self,
        instance:Option<InstanceHandle>,
        [x,y,width,height]:[i32;4],
        and_plane:*const u8,
        xor_plane:*const u8,
    )->Result<Cursor,Error>{
        unsafe{
            if let Some(cursor)=WinCore.cursor.create(
                instance,
                [x,y,width,height],
                and_plane,
                xor_plane
            ){
                Ok(
                    Self{
                        handle:cursor
                    }
                )
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Copies the specified cursor.
    /// 
    /// `cursor` - A handle to the cursor to be copied.
    /// 
    /// Enables an application or DLL to obtain the handle to a cursor shape owned by another module.
    /// Then if the other module is freed, the application is still able to use the cursor shape.
    /// 
    /// Do not use the `Cursor::copy` function for animated cursors.
    /// Instead, use the `CopyImage` function.
    /// 
    /// `Cursor::copy` is implemented as a call to the `Icon::copy` function.
    pub fn copy(&self)->Result<Cursor,Error>{
        unsafe{
            if let Some(cursor)=WinCore.cursor.copy(self.handle){
                Ok(
                    Self{
                        handle:cursor
                    }
                )
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Destroys a cursor and frees any memory the cursor occupied.
    /// Do not use this function to destroy a shared cursor.
    /// 
    /// The cursor must not be in use.
    /// 
    /// The `Cursor::destroy` function destroys a nonshared cursor.
    /// Do not use this function to destroy a shared cursor.
    /// A shared cursor is valid as long as the module from which it was loaded remains in memory.
    /// The following functions obtain a shared cursor:
    /// - `Cursor::load`
    /// - `Cursor::load_from_file`
    /// - `LoadImage` (if you use the `LR_SHARED` flag)
    /// - `CopyImage` (if you use the `LR_COPYRETURNORG` flag and the `hImage` parameter is a shared cursor)
    #[inline(always)]
    pub fn destroy(&self)->Result<(),Error>{
        unsafe{
            if WinCore.cursor.destroy(self.handle){
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    pub const fn handle(&self)->CursorHandle{
        self.handle
    }
}

impl Cursor{
    /// Loads the specified cursor resource from the executable (.EXE) file associated with an application instance.
    /// 
    /// `instance` - A handle to an instance of the module
    /// whose executable file contains the cursor to be loaded.
    /// 
    /// `name` - The name of the cursor resource to be loaded.
    /// 
    /// The `Cursor::load` function loads the cursor resource
    /// only if it has not been loaded; otherwise,
    /// it retrieves the handle to the existing resource.
    /// This function returns a valid cursor handle
    /// only if the `name` parameter is a pointer to a cursor resource.
    /// If `name` is a pointer to any type of resource other than a cursor (such as an icon),
    /// the return value is not `None`, even though it is not a valid cursor handle.
    /// 
    /// The `Cursor::load` function searches the cursor resource most appropriate for the cursor for the current display device.
    /// The cursor resource can be a color or monochrome bitmap.
    /// 
    /// This API does not participate in DPI virtualization.
    /// The output returned is not affected by the DPI of the calling thread.
    /// 
    /// This function has been superseded by the `LoadImage` function.
    #[inline(always)]
    pub fn load(instance:Option<InstanceHandle>,name:*const u16)->Result<Cursor,Error>{
        unsafe{
            if let Some(cursor)=WinCore.cursor.load(instance,name){
                Ok(
                    Self{
                        handle:cursor
                    }
                )
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Same as `Cursor::load` but for the ANSI encoding.
    #[inline(always)]
    pub fn load_ansi(instance:Option<InstanceHandle>,name:&str)->Result<Cursor,Error>{
        unsafe{
            if let Some(cursor)=WinCore.cursor.load_ansi(instance,name){
                Ok(
                    Self{
                        handle:cursor
                    }
                )
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Same as `Cursor::load` but for the system cursors.
    #[inline(always)]
    pub fn load_system_cursor(name:SystemCursor)->Result<Cursor,Error>{
        unsafe{
            if let Some(cursor)=WinCore.cursor.load_system_cursor(name){
                Ok(
                    Self{
                        handle:cursor
                    }
                )
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Creates a cursor based on data contained in a file.
    /// 
    /// `name` - The source of the file data to be used to create the cursor.
    /// The data in the file must be in either .CUR or .ANI format.
    /// If the high-order word of `name` is nonzero,
    /// it is a pointer to a string that is a fully qualified name of a file containing cursor data.
    /// 
    /// This API does not participate in DPI virtualization.
    /// The output returned is not affected by the DPI of the calling thread.
    #[inline(always)]
    pub fn load_from_file(name:*const u16)->Result<Cursor,Error>{
        unsafe{
            if let Some(cursor)=WinCore.cursor.load_from_file(name){
                Ok(
                    Self{
                        handle:cursor
                    }
                )
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Same as `Cursor::load_from_file` but for the ANSI encoding.
    #[inline(always)]
    pub fn load_from_file_ansi(name:*const u8)->Result<Cursor,Error>{
        unsafe{
            if let Some(cursor)=WinCore.cursor.load_from_file_ansi(name){
                Ok(
                    Self{
                        handle:cursor
                    }
                )
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }
}

impl Cursor{
    /// Sets the cursor shape.
    /// 
    /// `cursor` - A handle to the cursor.
    /// The cursor must have been created by the `Cursor::create` function
    /// or loaded by the `Cursor::load` or `LoadImage` function.
    /// If this parameter is `None`, the cursor is removed from the screen.
    /// 
    /// The return value is the handle to the previous cursor, if there was one.
    /// If there was no previous cursor, the return value is `None`.
    /// 
    /// The cursor is set only if the new cursor is different from the previous cursor;
    /// otherwise, the function returns immediately.
    /// 
    /// The cursor is a shared resource.
    /// A window should set the cursor shape only
    /// when the cursor is in its client area
    /// or when the window is capturing mouse input.
    /// In systems without a mouse, the window should restore the previous cursor
    /// before the cursor leaves the client area
    /// or before it relinquishes control to another window.
    /// 
    /// If your application must set the cursor while it is in a window,
    /// make sure the class cursor for the specified window's class is set to `None`.
    /// If the class cursor is not `None`,
    /// the system restores the class cursor each time the mouse is moved.
    /// 
    /// The cursor is not shown on the screen
    /// if the internal cursor display count is less than zero.
    /// This occurs if the application uses the `Cursor::show` function
    /// to hide the cursor more timesthan to show the cursor.
    #[inline(always)]
    pub fn set(cursor:Option<CursorHandle>)->Result<Cursor,Error>{
        unsafe{
            if let Some(cursor)=WinCore.cursor.set(cursor){
                Ok(
                    Self{
                        handle:cursor
                    }
                )
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Enables an application to customize the system cursors.
    /// It replaces the contents of the system cursor specified by the id parameter
    /// with the contents of the cursor specified by the `cursor` parameter and then destroys `cursor`.
    /// 
    /// `cursor` - A handle to the cursor.
    /// The function replaces the contents of the system cursor specified
    /// by id with the contents of the cursor handled by `cursor`.
    /// The system destroys `cursor` by calling the `Cursor::destroy` function.
    /// Therefore, `cursor` cannot be a cursor loaded using the `Cursor::load` function.
    /// To specify a cursor loaded from a resource, copy the cursor using the `Cursor::copy` function,
    /// then pass the copy to `Cursor::set_system`.
    /// 
    /// `id` - The system cursor to replace with the contents of `cursor`.
    /// This parameter can be one of the following values.
    #[inline(always)]
    pub fn set_system(cursor:Option<CursorHandle>,id:SystemCursorId)->Result<(),Error>{
        unsafe{
            if WinCore.cursor.set_system(cursor,id){
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Retrieves a handle to the current cursor.
    /// 
    /// To get information on the global cursor,
    /// even if it is not owned by the current thread, use `Cursor::get_info`.
    #[inline(always)]
    pub fn get()->Result<Cursor,Error>{
        unsafe{
            if let Some(cursor)=WinCore.cursor.get(){
                Ok(
                    Self{
                        handle:cursor
                    }
                )
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Retrieves information about the global cursor.
    /// 
    /// If the function succeeds, the return value is `true`.
    #[inline(always)]
    pub fn get_info(info:&mut CursorInfo)->Result<(),Error>{
        unsafe{
            if WinCore.cursor.get_info(info){
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Displays or hides the cursor.
    /// 
    /// This function sets an internal display counter that determines
    /// whether the cursor should be displayed.
    /// The cursor is displayed only if the display count is greater than or equal to 0.
    /// 
    /// If a mouse is installed, the initial display count is 0.
    /// If no mouse is installed, the display count is –1.
    #[inline(always)]
    pub fn show(show:bool)->i32{
        unsafe{
            WinCore.cursor.show(show)
        }
    }
}

impl Cursor{
    /// Confines the cursor to a rectangular area on the screen.
    /// If a subsequent cursor position (set by the `Cursor::set_position` function or the mouse) lies outside the rectangle,
    /// the system automatically adjusts the position to keep the cursor inside the rectangular area.
    /// 
    /// `rect` - A reference to the structure
    /// that contains the screen coordinates of the upper-left and lower-right corners of the confining rectangle.
    /// If this parameter is `None`, the cursor is free to move anywhere on the screen.
    /// 
    /// The cursor is a shared resource.
    /// If an application confines the cursor,
    /// it must release the cursor by using `Cursor::clip` before relinquishing control to another application.
    /// 
    /// The calling process must have `WINSTA_WRITEATTRIBUTES` access to the window station.
    #[inline(always)]
    pub fn clip(rect:Option<&[i32;4]>)->Result<(),Error>{
        unsafe{
            if WinCore.cursor.clip(rect){
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Retrieves the screen coordinates of the rectangular area to which the cursor is confined.
    /// 
    /// `rect` receives the screen coordinates of the confining rectangle,
    /// receives the dimensions of the screen if the cursor is not confined to a rectangle.
    /// 
    /// If the function succeeds, the return value is `true`.
    /// 
    /// The cursor is a shared resource.
    /// If an application confines the cursor with the `Cursor::clip` function,
    /// it must later release the cursor by using `Cursor::clip`
    /// before relinquishing control to another application.
    /// 
    /// The calling process must have `WINSTA_READATTRIBUTES` access to the window station.
    #[inline(always)]
    pub fn get_clip(rect:&mut [i32;4])->Result<(),Error>{
        unsafe{
            if WinCore.cursor.get_clip(rect){
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }
}


impl Cursor{
    /// Retrieves the position of the mouse cursor, in screen coordinates
    /// and writes it to `point`.
    /// 
    /// The cursor position is always specified in screen coordinates
    /// and is not affected by the mapping mode of the window that contains the cursor.
    /// 
    /// The calling process must have `WINSTA_READATTRIBUTES` access to the window station.
    /// 
    /// The input desktop must be the current desktop when you call `Cursor::get_position`.
    /// Call `OpenInputDesktop` to determine whether the current desktop is the input desktop.
    /// If it is not, call `SetThreadDesktop` with the `HDESK`
    /// returned by `OpenInputDesktop` to switch to that desktop.
    #[inline(always)]
    pub fn get_position(point:&mut [i32;2])->Result<(),Error>{
        unsafe{
            if WinCore.cursor.get_position(point){
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Moves the cursor to the specified screen coordinates.
    /// If the new coordinates are not within the screen rectangle set
    /// by the most recent `Cursor::clip` function call,
    /// the system automatically adjusts the coordinates so that the cursor stays within the rectangle.
    /// 
    /// The cursor is a shared resource.
    /// A window should move the cursor only when the cursor is in the window's client area.
    /// 
    /// The calling process must have `WINSTA_WRITEATTRIBUTES` access to the window station.
    /// 
    /// The input desktop must be the current desktop when you call `Cursor::set_position`.
    /// Call `OpenInputDesktop` to determine whether the current desktop is the input desktop.
    /// If it is not, call `SetThreadDesktop` with the `HDESK`
    /// returned by `OpenInputDesktop` to switch to that desktop.
    #[inline(always)]
    pub fn set_position([x,y]:[i32;2])->Result<(),Error>{
        unsafe{
            if WinCore.cursor.set_position([x,y]){
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Sets the position of the cursor in physical coordinates.
    /// 
    /// For a description of the difference
    /// between logical coordinates and physical coordinates, see `PhysicalToLogicalPoint`.
    #[inline(always)]
    pub fn set_physical_position([x,y]:[i32;2])->Result<(),Error>{
        unsafe{
            if WinCore.cursor.set_physical_position([x,y]){
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }

    /// Retrieves the position of the cursor in physical coordinates.
    /// 
    /// For a description of the difference
    /// between logical coordinates and physical coordinates, see `PhysicalToLogicalPoint`.
    #[inline(always)]
    pub fn get_physical_position(point:&mut [i32;2])->Result<(),Error>{
        unsafe{
            if WinCore.cursor.get_physical_position(point){
                Ok(())
            }
            else{
                Err(Error::get_last_error())
            }
        }
    }
}