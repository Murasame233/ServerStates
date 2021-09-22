#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::MacOS;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::Linux;

pub struct Count {
    total: u64,
    used: u64,
}

pub struct CountsAll {
    cores: Vec<Count>,
    all: Count,
}

pub trait Platform {
    fn init() -> Self;
    fn get_all(&self) -> &'_ [i32];
    fn update(&mut self);
    fn get(&self, core: usize) -> i32;
}

#[cfg(target_os = "macos")]
pub fn get_platform() -> impl Platform {
    MacOS::init()
}

#[cfg(target_os = "linux")]
pub fn get_platform() -> impl Platform {
    Linux::init()
}
