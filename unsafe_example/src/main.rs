struct UnsafeStruct {
    a: *mut u8,
    b: *mut u8,
}

impl UnsafeStruct {
    pub unsafe fn swap(&self) {
        unsafe {
            let tmp = *self.a;
            *self.a = *self.b;
            *self.b = tmp;
        }
    }
}

struct SafeStruct {
    inner_unsafe: UnsafeStruct,
}

impl SafeStruct {
    pub fn new(a: *mut u8, b: *mut u8) -> Self { 
        SafeStruct { inner_unsafe: UnsafeStruct { a, b } }
    }
}


fn main() {
    let mut a = 2u8;
    let mut b = 3u8;
    
    // &mut as *mut also possible to force the conversion into raw pointers
    let struct1 = SafeStruct::new(&mut a, &mut b);
    println!("a = {0:?}, b = {1:?}", unsafe{ *struct1.inner_unsafe.a}, unsafe{*struct1.inner_unsafe.b});
    
    unsafe {
        struct1.inner_unsafe.swap();
    }        
    println!("a = {0:?}, b = {1:?}", unsafe{*struct1.inner_unsafe.a}, unsafe{*struct1.inner_unsafe.b});
}
