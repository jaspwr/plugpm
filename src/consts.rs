pub const PLATFORM_BSD: &'static str = "MACOS";
pub const PLATFORM_LINUX: &'static str = "LINUX";
pub const PLATFORM_MACOS: &'static str = "MACOS";
pub const PLATFORM_WINDOWS: &'static str = "WINDOWS";

#[cfg(target_os = "windows")]
pub const THIS_PLATFORM: &'static str = PLATFORM_WINDOWS;

#[cfg(target_os = "macos")]
pub const THIS_PLATFORM: &'static str = PLATFORM_MACOS;

#[cfg(target_os = "linux")]
pub const THIS_PLATFORM: &'static str = PLATFORM_LINUX;

#[cfg(target_os = "windows")]
pub const VST2_INSTALL_PATH: &'static str = "C:\\Program Files\\Common Files\\VST2\\plugpm";
#[cfg(target_os = "windows")]
pub const VST3_INSTALL_PATH: &'static str = "C:\\Program Files\\Common Files\\VST3\\plugpm";
#[cfg(target_os = "windows")]
pub const CLAP_INSTALL_PATH: &'static str = "C:\\Program Files\\Common Files\\CLAP\\plugpm";

#[cfg(target_os = "macos")]
pub const VST2_INSTALL_PATH: &'static str = "/Library/Audio/Plug-Ins/VST2/plugpm";
#[cfg(target_os = "macos")]
pub const VST3_INSTALL_PATH: &'static str = "/Library/Audio/Plug-Ins/VST3/plugpm";
#[cfg(target_os = "macos")]
pub const CLAP_INSTALL_PATH: &'static str = "/Library/Audio/Plug-Ins/CLAP/plugpm";

pub const FORMAT_CLAP: &'static str = "CLAP";
pub const FORMAT_VST2: &'static str = "VST2";
pub const FORMAT_VST3: &'static str = "VST3";

pub const ARCH_AARCH64: &'static str = "AARCH64";
pub const ARCH_X86: &'static str = "X86";
pub const ARCH_X86_64: &'static str = "X86_64";

pub const CATEGORY_EQ: &'static str = "EQ";
pub const CATEGORY_COMPRESSOR: &'static str = "COMPRESSOR";
pub const CATEGORY_REVERB: &'static str = "REVERB";
pub const CATEGORY_DELAY: &'static str = "DELAY";
pub const CATEGORY_LIMITER: &'static str = "LIMITER";
pub const CATEGORY_SAMPLER: &'static str = "SAMPLER";
pub const CATEGORY_SATURATOR: &'static str = "SATURATOR";
pub const CATEGORY_DISTORTION: &'static str = "DISTORTION";
pub const CATEGORY_SYNTH: &'static str = "SYNTH";
pub const CATEGORY_SPECTRAL: &'static str = "SPECTRAL";
pub const CATEGORY_MIDI: &'static str = "MIDI";

