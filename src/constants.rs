#[cfg(target_os = "windows")]
pub const PATH_SDL2: &'static str = "SDL2.dll";
#[cfg(target_os = "linux")]
pub const PATH_SDL2: &'static str = "libSDL2-2.0.so.0";
