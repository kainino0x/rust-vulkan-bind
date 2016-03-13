use vulkan_bind::vk;

pub fn vkrq(res: vk::Result, msg: &str) {
    if res != vk::Result::SUCCESS {
        println!("fatal: {}: {:?}", msg, res);
        ::std::process::exit(1);
    }
}

pub fn vkrqr<T>(res: Result<T, vk::Result>, msg: &str) {
    if let Err(e) = res {
        vkrq(e, msg);
    }
}

pub fn vkwrap(res: vk::Result) -> Result<(), vk::Result> {
    if res == vk::Result::SUCCESS { Ok(()) } else { Err(res) }
}

pub fn vksuccess(res: vk::Result) {
    assert_eq!(vk::Result::SUCCESS, res);
}

#[macro_export]
macro_rules! vktry {
    ( $res:expr ) => { try!(::util::vkwrap($res)) }
}

pub fn cstr(s: &str) -> *const i8 {
    ::std::ffi::CString::new(s).unwrap().as_ptr()
}
