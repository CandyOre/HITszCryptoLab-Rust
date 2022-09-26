pub trait Demo {
    fn get_name(&self) -> String;
    fn start_demo(&mut self);
}