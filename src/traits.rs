//traits.rs
pub trait Sensor{
    fn name(&self) -> &str;
    fn read(&self) -> f32;
    fn is_faulty(&self) -> bool;
    fn inject_fault(&mut self);
    fn reset_fault(&mut self);
}