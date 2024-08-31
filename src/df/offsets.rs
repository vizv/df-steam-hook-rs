pub const GRAPHIC_SCREENX: usize = 0x84;
pub const GRAPHIC_SCREENF: usize = 0x8c; // TODO: check this on Windows
pub const GRAPHIC_SCREENF_UCCOLOR: usize = 0xcc; // TODO: check this on Windows

#[cfg(target_os = "linux")]
pub const RENDERER_DISPX_Z: usize = 0x160;
#[cfg(target_os = "windows")]
pub const RENDERER_DISPX_Z: usize = 0x168;
pub const RENDERER_SDL_RENDERER: usize = 0x108;
