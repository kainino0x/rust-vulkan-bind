use std;

#[derive(Clone, Eq, PartialEq, Debug)]
#[repr(i32)]
pub enum PipelineCacheHeaderVersion {
    One = 1,
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[repr(i32)]
pub enum Success {
    NotReady               =  1,
    Timeout                =  2,
    EventSet               =  3,
    EventReset             =  4,
    Incomplete             =  5,
    SuboptimalKHR          =  1000001003,
}

#[derive(Clone, Eq, PartialEq, Debug)]
#[repr(i32)]
pub enum Error {
    OutOfHostMemory        = -1,
    OutOfDeviceMemory      = -2,
    InitializationFailed   = -3,
    DeviceLost             = -4,
    MemoryMapFailed        = -5,
    LayerNotPresent        = -6,
    ExtensionNotPresent    = -7,
    FeatureNotPresent      = -8,
    IncompatibleDriver     = -9,
    TooManyObjects         = -10,
    FormatNotSupported     = -11,
    SurfaceLostKHR         = -1000000000,
    NativeWindowInUseKHR   = -1000000001,
    OutOfDateKHR           = -1000001004,
    IncompatibleDisplayKHR = -1000003001,
    ValidationFailedEXT    = -1000011001,
}

pub type Result = std::result::Result<Success, Error>;
