#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use libc::{c_uint, c_int, c_float, c_char, c_uchar, c_void, c_double, c_short, uint32_t, uintptr_t};
use std::ptr;
use std::default::Default;
use std::num::FromPrimitive;

// plain typedefs
pub type OvrBool = c_char;




//-----------------------------------------------------------------------------------
// ***** Simple Math Structures

/// A 2D vector with integer components.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct Vector2i {
  pub x: c_int,
  pub y: c_int,
}

/// A 2D size with integer components.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct Sizei {
  pub x: c_int,
  pub y: c_int,
}

/// A 2D rectangle with a position and size.
/// All components are integers.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct Recti {
  pub pos: Vector2i,
  pub size: Sizei,
}

/// A quaternion rotation.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct Quatf {
  pub x: c_float,
  pub y: c_float,
  pub z: c_float,
  pub w: c_float,
}

/// A 2D vector with float components.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct Vector2f {
  pub x: c_float,
  pub y: c_float,
}

/// A 3D vector with float components.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct Vector3f {
  pub x: c_float,
  pub y: c_float,
  pub z: c_float,
}

/// A 4x4 matrix with float elements.
//#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct Matrix4f {
  pub M: [[c_float, ..4], ..4],
}

/// Position and orientation together.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct Posef {
  pub Orientation: Quatf,
  pub Position: Vector3f,
}

/// A full pose (rigid body) configuration with first and second derivatives.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct PoseStatef {
  pub ThePose: Posef,
  pub AngularVelocity: Vector3f,
  pub LinearVelocity: Vector3f,
  pub AngularAcceleration: Vector3f,
  pub LinearAcceleration: Vector3f,
  pub TimeInSeconds: c_double,
}

/// Field Of View (FOV) in tangent of the angle units.
/// As an example, for a standard 90 degree vertical FOV, we would
/// have: { UpTan = tan(90 degrees / 2), DownTan = tan(90 degrees / 2) }.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct FovPort {
  pub UpTan: c_float,
  pub DownTan: c_float,
  pub LeftTan: c_float,
  pub RightTan: c_float,
}


//-----------------------------------------------------------------------------------
// ***** HMD Types

/// Enumerates all HMD types that we support.
pub const Hmd_None                         : c_uint = 0;
pub const Hmd_DK1                          : c_uint = 3;
pub const Hmd_DKHD                         : c_uint = 4;
pub const Hmd_DK2                          : c_uint = 6;
pub const Hmd_Other                        : c_uint = 7;

/// HMD capability bits reported by device.
pub const HmdCap_Present                   : c_uint = 0x0001;
pub const HmdCap_Available                 : c_uint = 0x0002;
pub const HmdCap_Captured                  : c_uint = 0x0004;
pub const HmdCap_ExtendDesktop             : c_uint = 0x0008;
pub const HmdCap_NoMirrorToWindow          : c_uint = 0x2000;
pub const HmdCap_DisplayOff                : c_uint = 0x0040;
pub const HmdCap_LowPersistence            : c_uint = 0x0080;
pub const HmdCap_DynamicPrediction         : c_uint = 0x0200;
pub const HmdCap_NoVSync                   : c_uint = 0x1000;

/// These bits can be modified by ovrHmd_SetEnabledCaps.
pub const HmdCap_Writable_Mask             : c_uint = 0x33F0;
/// These flags are currently passed into the service. May change without notice.
pub const HmdCap_Service_Mask              : c_uint = 0x23F0;
    

/// Tracking capability bits reported by the device.
/// Used with ovrHmd_ConfigureTracking.
pub const TrackingCap_Orientation          : c_uint = 0x0010;
pub const TrackingCap_MagYawCorrection     : c_uint = 0x0020;
pub const TrackingCap_Position             : c_uint = 0x0040;
pub const TrackingCap_Idle                 : c_uint = 0x0100;



/// Distortion capability bits reported by device.
/// Used with ovrHmd_ConfigureRendering and ovrHmd_CreateDistortionMesh.
pub const DistortionCap_Chromatic          : c_uint = 0x01;
pub const DistortionCap_TimeWarp           : c_uint = 0x02;
pub const DistortionCap_Vignette           : c_uint = 0x08;
pub const DistortionCap_NoRestore          : c_uint = 0x10;
pub const DistortionCap_FlipInput          : c_uint = 0x20;
pub const DistortionCap_SRGB               : c_uint = 0x40;
pub const DistortionCap_Overdrive          : c_uint = 0x80;
pub const DistortionCap_HqDistortion       : c_uint = 0x100;
pub const DistortionCap_LinuxDevFullscreen : c_uint = 0x200;
pub const DistortionCap_ProfileNoTimewarpSpinWaits : c_uint = 0x10000;


/// Specifies which eye is being used for rendering.
/// This type explicitly does not include a third "NoStereo" option, as such is
/// not required for an HMD-centered API.
pub const Eye_Left                         : c_uint = 0x00;
pub const Eye_Right                        : c_uint = 0x01;
pub const Eye_Count                        : c_uint = 0x02;


/// This is a complete descriptor of the HMD.
#[repr(C)]
pub struct HmdDesc {

  /// Internal handle of this HMD.
  pub Handle: *const Hmd,

  /// This HMD's type.  
  pub Type: c_int,

  /// Name string describing the product: "Oculus Rift DK1", etc.
  pub ProductName: *const c_char,
  pub Manufacturer: *const c_char,
  
  /// HID Vendor and ProductId of the device.
  pub VendorId: c_short,
  pub ProductId: c_short,
  /// Sensor (and display) serial number.
  pub SerialNumber: [c_char, ..24],
  /// Sensor firmware version.
  pub FirmwareMajor: c_short,
  pub FirmwareMinor: c_short,      
  /// External tracking camera frustum dimensions (if present).
  pub CameraFrustumHFovInRadians: c_float,
  pub CameraFrustumVFovInRadians: c_float,
  pub CameraFrustumNearZInMeters: c_float,
  pub CameraFrustumFarZInMeters: c_float,  
  
  /// Capability bits described by ovrHmdCaps.
  pub HmdCaps: c_uint,
	/// Capability bits described by ovrTrackingCaps.
  pub TrackingCaps: c_uint,
  /// Capability bits described by ovrDistortionCaps.
  pub DistortionCaps: c_uint,

  /// These define the recommended and maximum optical FOVs for the HMD.
  pub DefaultEyeFov: [FovPort, ..2],
  pub MaxEyeFov: [FovPort, ..2],
  
  /// Preferred eye rendering order for best performance.
  /// Can help reduce latency on sideways-scanned screens.
  pub EyeRenderOrder: [c_uint, ..2],

  /// Resolution of the full HMD screen (both eyes) in pixels.
  pub Resolution: Sizei,
  /// Location of the application window on the desktop (or 0,0).
  pub WindowsPos: Vector2i,

  /// Display that the HMD should present on.
  /// TBD: It may be good to remove this information relying on WindowPos instead.
  /// Ultimately, we may need to come up with a more convenient alternative,
  /// such as API-specific functions that return adapter, or something that will
  /// work with our monitor driver.
  /// Windows: (e.g. "\\\\.\\DISPLAY3", can be used in EnumDisplaySettings/CreateDC).
  pub DisplayDeviceName: *const c_char,
  /// MacOS:
  pub DisplayId: c_int,
}

pub enum Hmd {}

/// Bit flags describing the current status of sensor tracking.
pub const Status_OrientationTracked        : c_uint = 0x0001;
pub const Status_PositionTracked           : c_uint = 0x0002;
pub const Status_CameraPoseTracked         : c_uint = 0x0004;
pub const Status_PositionConnected         : c_uint = 0x0020;
pub const Status_HmdConnected              : c_uint = 0x0080;

/// Specifies a reading we can query from the sensor.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct SensorData {
  pub Accelerometer: Vector3f,
  pub Gyro: Vector3f,
  pub Magnetometer: Vector3f,
  pub Temperature: c_float,
  pub TimeInSeconds: c_float,
}


/// Tracking state at a given absolute time (describes predicted HMD pose etc).
/// Returned by ovrHmd_GetTrackingState.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct TrackingState {
  pub HeadPose: PoseStatef,
  pub CameraPose: Posef,
  pub LeveledCameraPose: Posef,
  pub RawSensorData: SensorData,
  pub StatusFlags: c_uint,
  pub LastVisionProcessingTime: c_double,
  pub LastVisionFrameLatency: c_double,
  pub LastCameraFrameCounter: uint32_t,
}


/// Frame timing data reported by ovrHmd_BeginFrameTiming() or ovrHmd_BeginFrame().
#[repr(C)]
pub struct FrameTiming {
  pub DeltaSeconds: c_float,
  pub ThisFrameSeconds: c_double,
  pub TimewarpPointSeconds: c_double,
  pub NextFrameSeconds: c_double,
  pub ScanoutMidpointSeconds: c_double,
  pub EyeScanoutSeconds: [c_double, ..2],
}

/// Rendering information for each eye. Computed by either ovrHmd_ConfigureRendering()
/// or ovrHmd_GetRenderDesc() based on the specified FOV. Note that the rendering viewport
/// is not included here as it can be specified separately and modified per frame through:
///    (a) ovrHmd_GetRenderScaleAndOffset in the case of client rendered distortion,
/// or (b) passing different values via ovrTexture in the case of SDK rendered distortion.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct EyeRenderDesc {
  pub Eye: c_uint,
  pub Fov: FovPort,
  pub DistortedViewport: Recti,
  pub PixelsPerTanAngleAtCenter: Vector2f,
  pub HmdToEyeViewOffset: Vector3f,
}

//-----------------------------------------------------------------------------------
// ***** Platform-independent Rendering Configuration

/// These types are used to hide platform-specific details when passing
/// render device, OS, and texture data to the API.
pub const RenderAPI_None                   : c_uint = 0;
pub const RenderAPI_OpenGL                 : c_uint = 1;
pub const RenderAPI_Android_GLES           : c_uint = 2;
pub const RenderAPI_D3D9                   : c_uint = 3;
pub const RenderAPI_D3D10                  : c_uint = 4;
pub const RenderAPI_D3D11                  : c_uint = 5;
pub const RenderAPI_Count                  : c_uint = 6;

/// Platform-independent part of rendering API-configuration data.
/// It is a part of ovrRenderAPIConfig, passed to ovrHmd_Configure.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct RenderAPIConfigHeader {
  pub API: c_uint,
  pub RTSize: Sizei,
  pub Multisample: c_int,
}

/// Contains platform-specific information for rendering.
#[repr(C)]
pub struct RenderAPIConfig {
  pub Header: RenderAPIConfigHeader,
  pub PlatformData: [uintptr_t, ..8],
}

/// Platform-independent part of the eye texture descriptor.
/// It is a part of ovrTexture, passed to ovrHmd_EndFrame.
/// If RenderViewport is all zeros then the full texture will be used.
#[repr(C)]
pub struct TextureHeader {
  pub API: c_uint,
  pub TextureSize: Sizei,
  pub RenderViewport: Recti, 
}

#[repr(C)]
pub struct Texture {
  pub Header: TextureHeader,
  pub PlatformData: [uintptr_t, ..8],
}

/// Used by ovrhmd_GetHSWDisplayState to report the current display state.
#[deriving(Clone, Default, Show)]
#[repr(C)]
pub struct HSWDisplayState {
  pub Displayed: OvrBool,
  pub StartTime: c_double,
  pub DismissibleTime: c_double,
}



extern "C" {
  pub fn ovr_InitializeRenderingShim();
  pub fn ovr_Initialize() -> OvrBool;
  pub fn ovr_Shutdown();
  pub fn ovr_GetVersionString() -> *const c_char;
  pub fn ovrHmd_Detect() -> c_int;
  pub fn ovrHmd_Create(index: c_int) -> *mut Hmd;
  pub fn ovrHmd_Destroy(hmd: *mut Hmd);
  pub fn ovrHmd_CreateDebug(hmd_type: c_uint) -> *mut Hmd;
  pub fn ovrHmd_GetLastError(hmd: *mut Hmd) -> *const char;
  pub fn ovrHmd_AttachToWindow(hmd: *mut Hmd, 
                               window: *mut c_void, 
                               destMirrorRect: *const Recti, 
                               sourceRenderTargetRect: *const Recti) -> OvrBool;
  pub fn ovrHmd_GetEnabledCaps(hmd: *mut Hmd) -> c_uint;
  pub fn ovrHmd_SetEnabledCaps(hmd: *mut Hmd, hmdCaps: c_uint);
  pub fn ovrHmd_ConfigureTracking(hmd: *mut Hmd, 
                                  supportedTrackingCaps: c_uint,
                                  requiredTrackingCaps: c_uint) -> OvrBool;
  pub fn ovrHmd_RecenterPose(hmd: *mut Hmd);
  pub fn ovrHmd_GetTrackingState(hmd: *mut Hmd, absTime: c_double) -> TrackingState;
  pub fn ovrHmd_GetFovTextureSize(hmd: *mut Hmd, 
                                  eye: c_uint, 
                                  fov: FovPort,
                                  pixelsPerDisplayPixel: c_float) -> Sizei;
  pub fn ovrHmd_ConfigureRendering(hmd: *mut Hmd,
                                   apiConfig: *const RenderAPIConfig,
                                   distortionCaps: c_uint,
                                   eyeFovIn: *const FovPort,
                                   eyeRenderDescOut: *mut EyeRenderDesc) -> OvrBool;
  pub fn ovrHmd_BeginFrame(hmd: *mut Hmd, frameIndex: c_uint) -> FrameTiming;
  pub fn ovrHmd_EndFrame(hmd: *mut Hmd,
                         renderPose: *const Posef,
                         eyeTexture: *const Texture);
  pub fn ovrHmd_GetEyePoses(hmd: *mut Hmd, 
                            frameIndex: c_uint, 
                            hmdToEyeViewOffset: *const Vector3f, // is not specified as const in the header, but should be?
                            outEyePoses: *mut Posef, 
                            outHmdTrackingState: *mut TrackingState);
  pub fn ovrHmd_GetRenderDesc(hmd: *mut Hmd, eyeType: c_uint, fov: FovPort) -> EyeRenderDesc;
  
  // TODO: Distortion mesh stuff omitted for the time being...
  
  pub fn ovrHmd_GetFrameTiming(hmd: *mut Hmd, frameIndex: c_uint) -> FrameTiming;
  pub fn ovrHmd_BeginFrameTiming(hmd: *mut Hmd, frameIndex: c_uint) -> FrameTiming;
  pub fn ovrHmd_EndFrameTiming(hmd: *mut Hmd);
  pub fn ovrHmd_ResetFrameTiming(hmd: *mut Hmd, frameIndex: c_uint);
  pub fn ovrHmd_GetEyeTimewarpMatrices(hmd: *mut Hmd, eye: c_uint, renderPose: Posef, twmOut: *mut Matrix4f);
  
//-------------------------------------------------------------------------------------
// ***** Stateless math setup functions

  pub fn ovrMatrix4f_Projection(fov: FovPort, znear: c_float, zfar: c_float, rightHanded: OvrBool) -> Matrix4f;
  pub fn ovrMatrix4f_OrthoSubProjection(projection: Matrix4f, 
                                        orthoScale: Vector2f,
                                        orthoDistance: c_float,
                                        hmdToEyeViewOffsetX: c_float) -> Matrix4f;
  pub fn ovr_GetTimeInSeconds() -> c_double;
  pub fn ovr_WaitTillTime(absTime: c_double) -> c_double;
  
// -----------------------------------------------------------------------------------
// ***** Latency Test interface

  pub fn ovrHmd_ProcessLatencyTest(hmd: *mut Hmd, rgbColorOut: *const c_uchar) -> OvrBool;  
  pub fn ovrHmd_GetLatencyTestResult(hmd: *mut Hmd) -> *const char;
  pub fn ovrHmd_GetLatencyTest2DrawColor(hmd: *mut Hmd, rgbColorOut: *const c_uchar);
  
//-------------------------------------------------------------------------------------
// ***** Health and Safety Warning Display interface
  
  pub fn ovrHmd_GetHSWDisplayState(hmd: *mut Hmd, hasWarningState: *mut HSWDisplayState);  
  pub fn ovrHmd_DismissHSWDisplay(hmd: *mut Hmd) -> OvrBool;
  

  pub fn ovrHmd_GetBool(hmd: *mut Hmd, propertyName: *const char, defaultVal: OvrBool) -> OvrBool;
  pub fn ovrHmd_SetBool(hmd: *mut Hmd, propertyName: *const char, value: OvrBool) -> OvrBool;

  pub fn ovrHmd_GetInt(hmd: *mut Hmd, propertyName: *const char, defaultVal: c_int) -> c_int;
  pub fn ovrHmd_SetInt(hmd: *mut Hmd, propertyName: *const char, value: c_int) -> OvrBool;  

  pub fn ovrHmd_GetFloat(hmd: *mut Hmd, propertyName: *const char, defaultVal: c_float) -> c_float;
  pub fn ovrHmd_SetFloat(hmd: *mut Hmd, propertyName: *const char, value: c_float) -> OvrBool;  
  
  pub fn ovrHmd_GetFloatArray(hmd: *mut Hmd, propertyName: *const char, values: *mut c_float, arraySize: c_uint) -> c_uint;
  pub fn ovrHmd_SetFloatArray(hmd: *mut Hmd, propertyName: *const char, values: *mut c_float, arraySize: c_uint) -> OvrBool;

  pub fn ovrHmd_GetString(hmd: *mut Hmd, propertyName: *const char, defaultVal: *const char) -> *const char;
  pub fn ovrHmd_SetString(hmd: *mut Hmd, propertyName: *const char, value: *const char) -> OvrBool;
  
// -----------------------------------------------------------------------------------
// ***** Logging  
  pub fn ovrHmd_StartPerfLog(hmd: *mut Hmd, fileName: *const char, userData1: *const char) -> OvrBool;
  pub fn ovrHmd_StopPerfLog(hmd: *mut Hmd) -> OvrBool;
  
}



/*

extern "C" {
    pub fn ovr_Initialize() -> bool;
    pub fn ovr_Shutdown();
    pub fn ovrHmd_Detect() -> c_int;
    pub fn ovrHmd_Create(index: c_int) -> *mut Hmd;
    pub fn ovrHmd_Destroy(hmd: *mut Hmd);
    pub fn ovrHmd_CreateDebug(hmd_type: c_int) -> *mut Hmd;
    pub fn ovrHmd_GetLastError(hmd: *mut Hmd) -> *const c_char;
    pub fn ovrHmd_GetEnabledCaps(hmd: *mut Hmd) -> c_uint;
    pub fn ovrHmd_SetEnabledCaps(hmd: *mut Hmd, flags: c_uint);
    pub fn ovrHmd_StartSensor(hmd: *mut Hmd,
                              supported: c_uint,
                              required: c_uint) -> bool;
    pub fn ovrHmd_StopSensor(hmd: *mut Hmd);
    pub fn ovrHmd_ResetSensor(hmd: *mut Hmd);
    pub fn ovrHmd_GetSensorState(hmd: *mut Hmd,
                                 abs_time: c_double) -> SensorState;
    pub fn ovrHmd_GetSensorDesc(hmd: *mut Hmd,
                                sensor_desc: *mut SensorDesc) -> bool;
    pub fn ovrHmd_GetDesc(hmd: *mut Hmd,
                          size: *mut HmdDesc);
    pub fn ovrHmd_GetFovTextureSize(hmd: *mut Hmd,
                                    eye: c_uint,
                                    fov: FovPort,
                                    pixels: c_float) -> Sizei;
    pub fn ovrHmd_ConfigureRendering(hmd: *mut Hmd,
                                     apiConfig: *const RenderApiConfig,
                                     distortionCaps: c_uint,
                                     fov_in: *const FovPort,
                                     render_desc_out: *mut EyeRenderDesc) -> bool;
    pub fn ovrHmd_BeginFrame(hmd: *mut Hmd,
                             frame_index: c_uint) -> FrameTiming;
    pub fn ovrHmd_EndFrame(hmd: *mut Hmd);
    pub fn ovrHmd_BeginEyeRender(hmd: *mut Hmd, eye: c_uint) -> Posef;
    pub fn ovrHmd_EndEyeRender(hmd: *mut Hmd, eye: c_uint, 
                               pose: Posef, texture: *const Texture);
    pub fn ovrMatrix4f_Projection(fov: FovPort,
                                  znear: c_float,
                                  zfar: c_float,
                                  right_handed: bool) -> Matrix4f;

    pub fn ovr_WaitTillTime(abs_time: c_double) -> c_double;
    pub fn ovr_GetTimeInSeconds() -> c_double;
}
*/












/*
/// Experiments with true enums to wrap the C enums, see:
/// http://stackoverflow.com/questions/19433827/is-it-possible-to-wrap-c-enums-in-rust
/// https://github.com/rust-lang/rust/issues/2132
/// https://github.com/rust-lang/rust/issues/3868
/// https://github.com/rust-lang/rust/pull/9250

#[deriving(PartialEq, FromPrimitive, Show)]
enum RenderAPI {
  RenderAPI_None         = 0,
  RenderAPI_OpenGL       = 1,
  RenderAPI_Android_GLES = 2,
  RenderAPI_D3D9         = 3,
  RenderAPI_D3D10        = 4,
  RenderAPI_D3D11        = 5,
  RenderAPI_Count        = 6,
}

#[test]
fn enumTests() {
  let x: Option<RenderAPI> = FromPrimitive::from_int(0);
  assert_eq!(x, Some(RenderAPI_None));
  //assert_eq!(FromPrimitive::from_int(0) as Option<RenderAPI>, Some(RenderAPI_None));
}


*/
