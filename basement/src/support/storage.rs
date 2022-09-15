pub struct DynamicStorage<T>{
    free_slots:Vec<usize>,
    slot_flags:Vec<bool>,
    slots:Vec<T>,
}

impl<T> DynamicStorage<T>{
    pub fn new()->DynamicStorage<T>{
        Self{
            free_slots:Vec::new(),
            slot_flags:Vec::new(),
            slots:Vec::new(),
        }
    }

    pub fn with_capacity(capacity:usize)->DynamicStorage<T>{
        Self{
            free_slots:Vec::with_capacity(capacity),
            slot_flags:Vec::with_capacity(capacity),
            slots:Vec::with_capacity(capacity),
        }
    }

    pub fn add(&mut self,item:T)->usize{
        unsafe{
            if let Some(slot)=self.free_slots.pop(){
                self.slot_flags[slot]=true;
                std::ptr::write(self.slots.get_unchecked_mut(slot),item);
                slot
            }
            else{
                let slot=self.slots.len();
                self.slot_flags.push(true);
                self.slots.push(item);
                slot
            }
        }
    }

    pub fn remove(&mut self,index:usize)->Option<T>{
        unsafe{
            if let Some(contains)=self.slot_flags.get_mut(index){
                if *contains{
                    self.free_slots.push(index);

                    *contains=!*contains;
                    return Some(std::ptr::read(self.slots.get_unchecked_mut(index)))
                }
            }
            None
        }
    }

    pub fn get(&self,index:usize)->Option<&T>{
        unsafe{
            if let Some(&contains)=self.slot_flags.get(index){
                if contains{
                    return Some(self.slots.get_unchecked(index))
                }
            }
            None
        }
    }

    pub fn get_mut(&mut self,index:usize)->Option<&mut T>{
        unsafe{
            if let Some(&contains)=self.slot_flags.get(index){
                if contains{
                    return Some(self.slots.get_unchecked_mut(index))
                }
            }
            None
        }
    }

    pub unsafe fn get_unchecked(&self,index:usize)->&T{
        self.slots.get_unchecked(index)
    }

    pub unsafe fn get_unchecked_mut(&mut self,index:usize)->&mut T{
        self.slots.get_unchecked_mut(index)
    }
}

impl<T> Drop for DynamicStorage<T>{
    fn drop(&mut self){
        unsafe{
            for (index,&flag) in self.slot_flags.iter().enumerate(){
                if flag{
                    let slot=&mut self.slots[index];
                    std::ptr::drop_in_place(slot)
                }
            }

            self.slots.set_len(0)
        }
    }
}

/// Статичное хранилище.
/// 
/// Даёт возможность использовать постоянные указатели на элемент в хранилище
/// без потери скорости обращения к нему.
pub struct StaticStorage<T>{
    free_slots:Vec<usize>,
    slot_flags:Vec<bool>,
    slots:Vec<T>,
}

impl<T> StaticStorage<T>{
    pub fn new(capacity:usize)->StaticStorage<T>{
        Self{
            free_slots:Vec::with_capacity(capacity),
            slot_flags:Vec::with_capacity(capacity),
            slots:Vec::with_capacity(capacity),
        }
    }

    pub fn add(&mut self,item:T)->Option<usize>{
        unsafe{
            if let Some(slot)=self.free_slots.pop(){
                self.slot_flags[slot]=true;
                std::ptr::write(self.slots.get_unchecked_mut(slot),item);
                Some(slot)
            }
            else{
                let slot=self.slots.len();
                if slot==self.slots.capacity(){
                    return None
                }
                self.slot_flags.push(true);
                self.slots.push(item);
                Some(slot)
            }
        }
    }

    pub fn remove(&mut self,index:usize)->Option<T>{
        unsafe{
            if let Some(contains)=self.slot_flags.get_mut(index){
                if *contains{
                    self.free_slots.push(index);

                    *contains=!*contains;
                    return Some(std::ptr::read(self.slots.get_unchecked_mut(index)))
                }
            }
            None
        }
    }

    pub fn get(&self,index:usize)->Option<&T>{
        unsafe{
            if let Some(&contains)=self.slot_flags.get(index){
                if contains{
                    return Some(self.slots.get_unchecked(index))
                }
            }
            None
        }
    }

    pub fn get_mut(&mut self,index:usize)->Option<&mut T>{
        unsafe{
            if let Some(&contains)=self.slot_flags.get(index){
                if contains{
                    return Some(self.slots.get_unchecked_mut(index))
                }
            }
            None
        }
    }

    pub unsafe fn get_unchecked(&self,index:usize)->&T{
        self.slots.get_unchecked(index)
    }

    pub unsafe fn get_unchecked_mut(&mut self,index:usize)->&mut T{
        self.slots.get_unchecked_mut(index)
    }
}

impl<T> Drop for StaticStorage<T>{
    fn drop(&mut self){
        unsafe{
            for (index,&flag) in self.slot_flags.iter().enumerate(){
                if flag{
                    let slot=&mut self.slots[index];
                    std::ptr::drop_in_place(slot)
                }
            }

            self.slots.set_len(0)
        }
    }
}