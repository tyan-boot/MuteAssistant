use anyhow::Result;

use winapi::um::combaseapi::*;
use winapi::um::endpointvolume::*;
use winapi::um::mmdeviceapi::*;

use winapi::shared::winerror::FAILED;
use winapi::um::objbase::CoInitialize;
use winapi::Interface;

use std::ptr;
use std::ptr::NonNull;

macro_rules! check_result {
    ( $result:ident, $reason:expr ) => {
        if FAILED($result) {
            return Err(anyhow::anyhow!($reason));
        }
    };
}

#[allow(dead_code)]
pub struct Volume {
    enumerator: NonNull<IMMDeviceEnumerator>,

    default_device: NonNull<IMMDevice>,

    volume_endpoint: NonNull<IAudioEndpointVolume>,
}

impl Volume {
    pub fn new() -> Result<Self> {
        let r = unsafe { CoInitialize(ptr::null_mut()) };
        check_result!(r, "CoInitialize failed");

        let mut enumerator = ptr::null_mut();
        let r = unsafe {
            CoCreateInstance(
                &CLSID_MMDeviceEnumerator,
                ptr::null_mut(),
                CLSCTX_ALL,
                &IMMDeviceEnumerator::uuidof(),
                &mut enumerator,
            )
        };
        check_result!(r, "Create IMMDeviceEnumerator instance failed");
        let enumerator: NonNull<IMMDeviceEnumerator> =
            NonNull::new(enumerator as *mut IMMDeviceEnumerator).unwrap();

        let mut default_device = ptr::null_mut();
        let r = unsafe {
            enumerator
                .as_ref()
                .GetDefaultAudioEndpoint(1, 0, &mut default_device)
        };
        check_result!(r, "GetDefaultAudioEndpoint failed");
        let default_device: NonNull<IMMDevice> = NonNull::new(default_device).unwrap();

        let mut volume_endpoint = ptr::null_mut();
        let r = unsafe {
            default_device.as_ref().Activate(
                &IAudioEndpointVolume::uuidof(),
                CLSCTX_ALL,
                ptr::null_mut(),
                &mut volume_endpoint,
            )
        };
        check_result!(r, "Create IAudioEndpointVolume instance failed");
        let volume_endpoint = NonNull::new(volume_endpoint as *mut IAudioEndpointVolume).unwrap();

        Ok(Volume {
            enumerator,
            default_device,
            volume_endpoint,
        })
    }

    pub fn mute(&self) -> bool {
        let r = unsafe { self.volume_endpoint.as_ref().SetMute(1, ptr::null_mut()) };
        !FAILED(r)
    }

    pub fn unmute(&self) -> bool {
        let r = unsafe { self.volume_endpoint.as_ref().SetMute(0, ptr::null_mut()) };
        !FAILED(r)
    }

    pub fn toggle(&self) {
        let mut m = 0;

        unsafe {
            self.volume_endpoint.as_ref().GetMute(&mut m);
            dbg!(m);
            dbg!((m == 0) as i32);
            self.volume_endpoint
                .as_ref()
                .SetMute((m == 0) as i32, ptr::null_mut());
        }
    }

    pub fn is_mute(&self) -> bool {
        let mut m = 0;
        unsafe {
            self.volume_endpoint.as_ref().GetMute(&mut m);
        }

        m == 1
    }
}
