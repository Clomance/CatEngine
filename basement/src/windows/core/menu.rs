use winapi::{
    shared::windef::HMENU,

    um::wingdi::{

    },
};

use core::{
    mem::{
        transmute,
        transmute_copy,
        size_of,
    },
    ptr::NonNull,
};

#[derive(Clone,Copy)]
#[repr(transparent)]
pub struct MenuHandle{
    inner:NonNull<()>,
}
implement_handle_wrapper!(MenuHandle,HMENU);

pub struct Menu;

impl Menu{
    pub const fn new()->Menu{
        Self
    }
}