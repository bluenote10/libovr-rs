#![allow(dead_code)]

use super::ffi;
use libc::{c_int, c_uint, c_void, c_float, c_double};
use std::string::raw::from_buf;
use std::default::Default;

pub use ffi::TrackingState;
pub use ffi::FovPort;
pub use ffi::Sizei;
pub use ffi::RenderAPIConfig;
pub use ffi::RenderAPIConfigHeader;
pub use ffi::EyeRenderDesc;
pub use ffi::FrameTiming;
pub use ffi::Vector3f;
pub use ffi::Posef;

//-----------------------------------------------------------------------------------
// Enum wrappers
//-----------------------------------------------------------------------------------

#[deriving(PartialEq, Show)]
pub enum HmdType {
  HmdNone,
  HmdDK1,
  HmdDKHD,
  HmdDK2,
  HmdOther,
}
impl HmdType {
  fn from_ffi(c: c_uint) -> HmdType {
    match c {
      ffi::Hmd_None       => HmdNone,
      ffi::Hmd_DK1        => HmdDK1,
      ffi::Hmd_DKHD       => HmdDKHD,
      ffi::Hmd_DK2        => HmdDK2,
      _                   => HmdOther  
    }
  }
  fn to_ffi(&self) -> c_uint {
    match *self {
      HmdNone             => ffi::Hmd_None,
      HmdDK1              => ffi::Hmd_DK1,
      HmdDKHD             => ffi::Hmd_DKHD,
      HmdDK2              => ffi::Hmd_DK2,
      HmdOther            => ffi::Hmd_Other 
    }
  }
}

#[deriving(PartialEq, Show)]
pub enum EyeType {
  EyeL,
  EyeR,
}

impl EyeType {
  fn from_ffi(c: c_uint) -> EyeType {
    match c {
      ffi::Eye_Left  => EyeL,
      ffi::Eye_Right => EyeR,
      _ => panic!("Invalid eye type {}", c)
    }
  }
  fn to_ffi(&self) -> c_uint {
    match *self {
      EyeL => ffi::Eye_Left,
      EyeR => ffi::Eye_Right
    }
  }
}


//-----------------------------------------------------------------------------------
// Bitset wrappers
//-----------------------------------------------------------------------------------

pub struct BitMask {
  pub mask: c_uint,
}
impl BitMask {
  fn check_flag(&self, flag: c_uint) -> bool {
    self.mask & flag == flag
  }
  fn set_flag(&self, flag: c_uint, new_state: bool) -> BitMask {
    BitMask { mask:
      match new_state {
        true  => self.mask |  flag,
        false => self.mask & !flag,
      }
    }
  }
}

pub type HmdCaps = BitMask;
pub type DistortionCaps = BitMask;
pub type TrackingCaps = BitMask;

/*
/// Eventually, a type-safe approach like the following is better as the approach above.
/// However, since all the bitmasks are still subject to change in the SDK,
/// I chose a more low level approach for the time being...

#[deriving(Show)]
pub struct HmdCaps {
  flags: c_uint
}

impl HmdCaps {
  // non setable flags
  pub fn present(&self) -> bool {
    self.flags & ffi::HmdCap_Present == ffi::HmdCap_Present
  }
  pub fn available(&self) -> bool {
    self.flags & ffi::HmdCap_Available == ffi::HmdCap_Available
  }
  pub fn captured(&self) -> bool {
    self.flags & ffi::HmdCap_Captured == ffi::HmdCap_Captured
  }  
  pub fn extend_desktop(&self) -> bool {
    self.flags & ffi::HmdCap_ExtendDesktop == ffi::HmdCap_ExtendDesktop
  }  
  
  // setable flags
  pub fn no_mirror_to_window(&self) -> bool {
    self.flags & ffi::HmdCap_NoMirrorToWindow == ffi::HmdCap_NoMirrorToWindow
  }
  pub fn set_no_mirror_to_window(&self, flag: bool) -> HmdCaps {
    HmdCaps { flags: 
      match flag {
        true  => self.flags |  ffi::HmdCap_NoMirrorToWindow,
        false => self.flags & !ffi::HmdCap_NoMirrorToWindow,
      }
    }
  }

  pub fn display_off(&self) -> bool {
    self.flags & ffi::HmdCap_DisplayOff == ffi::HmdCap_DisplayOff
  }
  pub fn set_display_off(&self, flag: bool) -> HmdCaps {
    HmdCaps { flags: 
      match flag {
        true  => self.flags |  ffi::HmdCap_DisplayOff,
        false => self.flags & !ffi::HmdCap_DisplayOff,
      }
    }
  }
  
  pub fn low_persistance(&self) -> bool {
    self.flags & ffi::HmdCap_LowPersistence == ffi::HmdCap_LowPersistence
  }
  pub fn set_low_persistance(&self, flag: bool) -> HmdCaps {
    HmdCaps { flags: 
      match flag {
        true  => self.flags |  ffi::HmdCap_LowPersistence,
        false => self.flags & !ffi::HmdCap_LowPersistence,
      }
    }
  }

  pub fn dynamic_prediction(&self) -> bool {
    self.flags & ffi::HmdCap_DynamicPrediction == ffi::HmdCap_DynamicPrediction
  }
  pub fn set_dynamic_prediction(&self, flag: bool) -> HmdCaps {
    HmdCaps { flags: 
      match flag {
        true  => self.flags |  ffi::HmdCap_DynamicPrediction,
        false => self.flags & !ffi::HmdCap_DynamicPrediction,
      }
    }
  }
  
  pub fn no_vsync(&self) -> bool {
    self.flags & ffi::HmdCap_NoVSync == ffi::HmdCap_NoVSync
  }
  pub fn set_no_vsync(&self, flag: bool) -> HmdCaps {
    HmdCaps { flags: 
      match flag {
        true  => self.flags |  ffi::HmdCap_NoVSync,
        false => self.flags & !ffi::HmdCap_NoVSync,
      }
    }
  }  
}
*/


//-----------------------------------------------------------------------------------
// Wrapper for general OVR library
//-----------------------------------------------------------------------------------

pub struct Ovr;

impl Ovr {

  pub fn initialize() -> Option<Ovr> {
    unsafe {
      if ffi::ovr_Initialize() != 0i8 {
        Some(Ovr)
      } else {
        None
      }
    }
  }

  pub fn detect(&self) -> int {
    unsafe { ffi::ovrHmd_Detect() as int }
  }

  pub fn create_hmd(&self, index: int) -> Option<Hmd> {
    unsafe {
      let ptr = ffi::ovrHmd_Create(index as i32);
      if !ptr.is_null() {
        Some(Hmd{ ptr: ptr })
      } else {
        None
      }
    }
  }

  pub fn create_hmd_debug(&self, hmd_type: HmdType) -> Option<Hmd> {
    unsafe {
      let ptr = ffi::ovrHmd_CreateDebug(hmd_type.to_ffi());
      if !ptr.is_null() {
        Some(Hmd{ ptr: ptr })
      } else {
        None
      }
    }   
  }
  
}

impl Drop for Ovr {
  fn drop(&mut self) {
    unsafe { ffi::ovr_Shutdown(); }
  }
}


//-----------------------------------------------------------------------------------
// Wrapper for HMD
//-----------------------------------------------------------------------------------

pub struct Hmd {
  ptr: *mut ffi::Hmd
}


impl Hmd {

  pub fn get_last_error(&self) -> Option<String> {
    unsafe {
      let ptr = ffi::ovrHmd_GetLastError(self.ptr);
      match ptr.is_null() {
        true  => None,
        false => Some(from_buf(ptr as *const u8)),
      }
    }
  }

  /// TODO
  pub fn attach_to_window() {
    unimplemented!()
  }

  pub fn get_enabled_caps(&self) -> HmdCaps {
    unsafe {
      let flags = ffi::ovrHmd_GetEnabledCaps(self.ptr);
      HmdCaps{ mask: flags }
    }
  }

  pub fn set_enabled_caps(&self, caps: HmdCaps) {
    unsafe {
      let flags = caps.mask;
      ffi::ovrHmd_SetEnabledCaps(self.ptr, flags);
    }
  }

  pub fn configure_tracking(&self, supported_tracking_caps: TrackingCaps, required_tracking_caps: TrackingCaps) -> bool {
    unsafe {
      let ovr_bool = ffi::ovrHmd_ConfigureTracking(
        self.ptr, supported_tracking_caps.mask, required_tracking_caps.mask
      );
      ovr_bool != 0i8
    }
  }

  pub fn recenter_pose(&self) {
    unsafe {
      ffi::ovrHmd_RecenterPose(self.ptr);
    }
  }

  pub fn get_tracking_state(&self, abs_time: f64) -> TrackingState {
    unsafe {
      ffi::ovrHmd_GetTrackingState(self.ptr, abs_time)
    }
  }

  pub fn get_fov_texture_size(&self, eye: EyeType, fov: FovPort, pixels_per_display_pixel: f32) -> Sizei {
    unsafe {
      ffi::ovrHmd_GetFovTextureSize(self.ptr, eye.to_ffi(), fov, pixels_per_display_pixel)
    }
  }
  
  pub fn configure_rendering(
    &self, 
    api_config: RenderAPIConfig,
    distortion_caps: DistortionCaps,
    eye_fov_in: [FovPort, ..2],     
  ) -> [EyeRenderDesc, ..2] {
    unsafe {
      let eye_fov_in_ptr: *const FovPort = &eye_fov_in[0];
      let mut eye_render_desc_out: [EyeRenderDesc, ..2] = [Default::default(), Default::default()];
      let eye_render_desc_out_ptr: *mut EyeRenderDesc = &mut eye_render_desc_out[0];
      ffi::ovrHmd_ConfigureRendering(self.ptr, &api_config, distortion_caps.mask, eye_fov_in_ptr, eye_render_desc_out_ptr);
      eye_render_desc_out
    }
  }

  pub fn begin_frame(&self, frame_index: i32) -> FrameTiming {
    unsafe {
      ffi::ovrHmd_BeginFrame(self.ptr, frame_index as c_uint)
    }
  }

  pub fn get_eye_poses(&self, frame_index: i32, hmd_to_eye_view_offset: [Vector3f, ..2]) {
    unsafe {
      let mut out_eye_poses: [Posef, ..2] = [Default::default(), Default::default()];
      let out_eye_poses_ptr: *mut Posef = &mut out_eye_poses[0];
      let mut out_hmd_tracking_state: TrackingState = Default::default();
      ffi::ovrHmd_GetEyePoses(self.ptr,
                              frame_index as c_uint,
                              &hmd_to_eye_view_offset[0],
                              out_eye_poses_ptr,
                              &mut out_hmd_tracking_state);
    }
  }

/*
  pub fn ovrHmd_GetEyePoses(hmd: *mut Hmd, 
                            frameIndex: c_uint, 
                            hmdToEyeViewOffset: *mut Posef,
                            outEyePoses: *mut Posef, 
                            outHmdTrackingState: *mut TrackingState);


    pub fn get_sensor_state(&self, abs_time: f64) -> SensorState {
        unsafe {
            SensorState::from_ll(ll::ovrHmd_GetSensorState(self.ptr, abs_time))
        }
    }

    pub fn get_sensor_description(&self) -> Option<SensorDescription> {
        unsafe {
            let mut c_desc = ll::SensorDesc {
                vendor_id: 0,
                product_id: 0,
                serial_number: [0,.. 24]
            };

            if !ll::ovrHmd_GetSensorDesc(self.ptr, &mut c_desc as *mut ll::SensorDesc) {
                None
            } else {
                Some(SensorDescription::from_ll(c_desc))
            }
        }
    }

    pub fn get_description(&self) -> HmdDescription {
        unsafe {
            let mut c_desc = Default::default();
            ll::ovrHmd_GetDesc(self.ptr, &mut c_desc);
            HmdDescription::from_ll(c_desc)
        }
    }

    pub fn get_fov_texture_size(&self,
                                eye: EyeType,
                                fov: FovPort,
                                pixels_per_display_pixel: f32) -> ll::Sizei {
        unsafe {
            ll::ovrHmd_GetFovTextureSize(self.ptr,
                                         eye.to_ll(),
                                         fov.to_ll(),
                                         pixels_per_display_pixel)
        }
    }

    pub fn configure_rendering<RC: ToRenderConfig>(&self,
                               api_config: &RC,
                               cap: DistortionCapabilities,
                               eye_fov: PerEye<FovPort>) -> Option<PerEye<EyeRenderDescriptor>> {
        unsafe {
            let mut out: PerEye<ll::EyeRenderDesc> = PerEye::new(Default::default(),
                                                                 Default::default());
            let was_started = ll::ovrHmd_ConfigureRendering(
                self.ptr,
                &api_config.to_render_config(),
                cap.flags,
                eye_fov.map(|_, d| d.to_ll()).ptr(),
                out.mut_ptr()
            );

            if was_started {
                Some(out.map(|_, d| EyeRenderDescriptor::from_ll(d)))
            } else {
                None
            }
        }
    }

    pub fn begin_frame(&self, frame_index: uint) -> FrameTiming {
        unsafe {
            FrameTiming::from_ll(
                ll::ovrHmd_BeginFrame(self.ptr, frame_index as c_uint)
            )
        }
    }

    pub fn end_frame(&self) {
        unsafe {
            ll::ovrHmd_EndFrame(self.ptr);
        }
    }

    pub fn begin_eye_render(&self, eye: EyeType) -> Pose {
        unsafe {
            Pose::from_ll(ll::ovrHmd_BeginEyeRender(self.ptr, eye.to_ll()))
        }
    }

    pub fn end_eye_render<T: ToTexture>(&self,
                                        eye: EyeType,
                                        pose: Pose,
                                        texture: &T) {
        unsafe {
            ll::ovrHmd_EndEyeRender(self.ptr,
                                    eye.to_ll(),
                                    pose.to_ll(),
                                    &texture.to_texture());
        }
    }
    */
}

impl Drop for Hmd {
  fn drop(&mut self) {
    unsafe {ffi::ovrHmd_Destroy(self.ptr)}
  }
}




