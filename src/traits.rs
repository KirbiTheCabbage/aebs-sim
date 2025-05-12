//traits.rs
use std::any::Any;

pub trait Sensor: Any {
    fn name(&self) -> &str;
    fn read(&self) -> f32;
    fn is_faulty(&self) -> bool;
    fn inject_fault(&mut self);
    fn reset_fault(&mut self);


    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
