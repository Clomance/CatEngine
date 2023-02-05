pub mod client;

pub mod device;

pub mod host;

use winapi::{
    shared::winerror::{
        E_INVALIDARG,
        E_POINTER,
        E_OUTOFMEMORY
    },
    um::audioclient::{
        AUDCLNT_E_ALREADY_INITIALIZED,
        AUDCLNT_E_WRONG_ENDPOINT_TYPE,
        AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED,
        AUDCLNT_E_BUFFER_SIZE_ERROR,
        AUDCLNT_E_CPUUSAGE_EXCEEDED,
        AUDCLNT_E_INVALID_SIZE,
        AUDCLNT_E_OUT_OF_ORDER,
        AUDCLNT_E_DEVICE_INVALIDATED,
        AUDCLNT_E_SERVICE_NOT_RUNNING,
        AUDCLNT_E_DEVICE_IN_USE,
        AUDCLNT_E_ENDPOINT_CREATE_FAILED,
        AUDCLNT_E_INVALID_DEVICE_PERIOD,
        AUDCLNT_E_UNSUPPORTED_FORMAT,
        AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED,
        AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL,
        AUDCLNT_E_BUFFER_ERROR,
        AUDCLNT_E_BUFFER_TOO_LARGE,
        AUDCLNT_E_BUFFER_OPERATION_PENDING,
        AUDCLNT_E_NOT_INITIALIZED
    },
};



#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum AudioClientError{
    /// For unknown bug error.
    /// 
    /// May accure when the client buffer is retrieved too frequently.
    None=0,

    NotInitiated=AUDCLNT_E_NOT_INITIALIZED,
    AlreadyInitialized=AUDCLNT_E_ALREADY_INITIALIZED,
    WrongEndpointType=AUDCLNT_E_WRONG_ENDPOINT_TYPE,
    CpuUsageExceeded=AUDCLNT_E_CPUUSAGE_EXCEEDED,
    InvalidSize=AUDCLNT_E_INVALID_SIZE,

    InvalidArg=E_INVALIDARG,

    EndpointCreateFailed=AUDCLNT_E_ENDPOINT_CREATE_FAILED,

    /// Indicates that the device period requested by an exclusive-mode client is greater than 5000 milliseconds.
    /// 
    /// Applies to Windows 7 and later.
    InvalidDevicePeriod=AUDCLNT_E_INVALID_DEVICE_PERIOD,
    UnsupportedFormat=AUDCLNT_E_UNSUPPORTED_FORMAT,
    ExclusiveModeNotAllowed=AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED,

    NullPointerFormat=E_POINTER,

    OutOfOrder=AUDCLNT_E_OUT_OF_ORDER,
    OutOfMemory=E_OUTOFMEMORY,

    DeviceInvalidated=AUDCLNT_E_DEVICE_INVALIDATED,
    DeviceInUse=AUDCLNT_E_DEVICE_IN_USE,

    ServiceNotRunning=AUDCLNT_E_SERVICE_NOT_RUNNING,

    BufferDurationPeriodNotEqual=AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL,
    BufferSizeNotAligned=AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED,
    BufferSizeError=AUDCLNT_E_BUFFER_SIZE_ERROR,
    BufferError=AUDCLNT_E_BUFFER_ERROR,
    BufferTooLarge=AUDCLNT_E_BUFFER_TOO_LARGE,
    BufferOperationPending=AUDCLNT_E_BUFFER_OPERATION_PENDING
}

impl AudioClientError{
    pub (crate) fn new(value:i32)->AudioClientError{
        println!("Transmute {}",value);
        unsafe{
            std::mem::transmute(value)
        }
    }
}