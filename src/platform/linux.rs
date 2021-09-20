use super::Platform;

pub struct Linux{

}

impl Platform for Linux {
    fn init()->Self {
        todo!()
    }

    fn get_all(&self)->&'_ [i32] {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn get(&self,core:usize)->i32 {
        todo!()
    }
}