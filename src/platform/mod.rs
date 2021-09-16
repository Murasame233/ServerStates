pub mod macos;
pub use macos::MacOS;

pub trait Platform {
    fn init()->Self;
    fn get_all(&self)->&'_ [i32];
    fn update(&mut self);
    fn get(&self,core:usize)->i32;
}