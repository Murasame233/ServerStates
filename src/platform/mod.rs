#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::MacOS;

pub trait Platform {
    fn init()->Self;
    fn get_all(&self)->&'_ [i32];
    fn update(&mut self);
    fn get(&self,core:usize)->i32;
}

pub fn get_platform()-> impl Platform {
    if cfg!(target_os="macos") {
        MacOS::init()
    } else {
        todo!()
    }
}