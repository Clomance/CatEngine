use winapi::{
    shared::windef::HMENU,
};

use core::{
    mem::{
        transmute,
        transmute_copy,
    },
    ptr::NonNull,
};

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct MenuHandle(NonNull<()>);
implement_handle_wrapper!(MenuHandle,HMENU);

pub struct Menu;