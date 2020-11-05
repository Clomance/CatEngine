pub mod math;

/// Сырой указатель,
/// который можно передавать
/// по потокам.
#[derive(Clone)]
pub struct SyncRawPtr<T>{
    ptr:*const T,
}

impl<T> SyncRawPtr<T>{
    pub fn new(item:&T)->SyncRawPtr<T>{
        Self{
            ptr:item as *const T,
        }
    }

    pub fn zero()->SyncRawPtr<T>{
        Self{
            ptr:0u64 as *const T,
        }
    }

    pub fn offset(&mut self,value:isize){
        unsafe{
            self.ptr=self.ptr.offset(value);
        }
    }
}

unsafe impl<T> std::marker::Send for SyncRawPtr<T>{}
unsafe impl<T> std::marker::Sync for SyncRawPtr<T>{}

impl<T> AsRef<T> for SyncRawPtr<T>{
    fn as_ref(&self)->&T{
        unsafe{
            &*self.ptr
        }
    }
}

/// Сырой указатель,
/// который можно передавать
/// по потокам и который позволяет
/// менять данные.
#[derive(Clone)]
pub struct SyncRawMutPtr<T>{
    ptr:*mut T,
}


impl<T> SyncRawMutPtr<T>{
    pub fn new(item:&mut T)->SyncRawMutPtr<T>{
        Self{
            ptr:item as *mut T,
        }
    }

    pub fn zero()->SyncRawMutPtr<T>{
        Self{
            ptr:0u64 as *mut T,
        }
    }

    pub fn offset(&mut self,value:isize){
        unsafe{
            self.ptr=self.ptr.offset(value);
        }
    }
}

unsafe impl<T> std::marker::Send for SyncRawMutPtr<T>{}
unsafe impl<T> std::marker::Sync for SyncRawMutPtr<T>{}

impl<T> AsRef<T> for SyncRawMutPtr<T>{
    fn as_ref(&self)->&T{
        unsafe{
            &*self.ptr
        }
    }
}

impl<T> AsMut<T> for SyncRawMutPtr<T>{
    fn as_mut(&mut self)->&mut T{
        unsafe{
            &mut *self.ptr
        }
    }
}