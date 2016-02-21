use vk::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearColorValue {
    _bindgen_data_: [u32; 4usize],
}
impl ClearColorValue {
    pub unsafe fn float32(&mut self)
     -> *mut [::std::os::raw::c_float; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn int32(&mut self) -> *mut [i32; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn uint32(&mut self) -> *mut [u32; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearValue {
    _bindgen_data_: [u32; 4usize],
}
impl ClearValue {
    pub unsafe fn color(&mut self) -> *mut ClearColorValue {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn depthStencil(&mut self) -> *mut ClearDepthStencilValue {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
