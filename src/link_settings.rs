
#[cfg(target_os = "linux")]
#[link(name="ovr")]
#[link(name="stdc++")]
#[link(name="udev")]
#[link(name="Xinerama")]
#[link(name="Xrandr")]
#[link(name="X11")]
#[link(name="GL")]
extern {}
 
#[cfg(target_os = "macos")]
#[link(name="ovr")]
#[link(name="stdc++")]
#[link(name = "Cocoa", kind = "framework")]
#[link(name = "IOKit", kind = "framework")]
#[link(name = "CoreFoundation", kind = "framework")]
extern {}

