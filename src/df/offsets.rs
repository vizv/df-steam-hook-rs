#[cfg(target_os = "linux")]
pub const ENABLER_TEXTURES: usize = 0x388;
#[cfg(target_os = "windows")]
pub const ENABLER_TEXTURES: usize = 0x348;

pub const GAME_MAIN_INTERFACE_HELP: usize = 0x5d40; // TODO: check this on Windows

pub const GRAPHIC_SCREENX: usize = 0x84;
pub const GRAPHIC_SCREENF: usize = 0x8c; // TODO: check this on Windows
pub const GRAPHIC_SCREENF_UCCOLOR: usize = 0xcc; // TODO: remove this, use GRAPHIC_UCCOLOR instead
pub const GRAPHIC_UCCOLOR: usize = 0x158; // TODO: check this on Windows
#[cfg(target_os = "linux")]
pub const GRAPHIC_DIMX: usize = 0xa00;
#[cfg(target_os = "windows")]
pub const GRAPHIC_DIMX: usize = 0x6cc;

pub const RENDERER_SDL_RENDERER: usize = 0x108;
#[cfg(target_os = "linux")]
pub const RENDERER_DISPX_Z: usize = 0x160;
#[cfg(target_os = "windows")]
pub const RENDERER_DISPX_Z: usize = 0x168;
