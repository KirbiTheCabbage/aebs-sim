// sensors.rs 
use crate::traits::Sensor;

pub struct LidarSensor{
    pub distance: f32,
    pub faulty: bool,
}

impl LidarSensor {
    pub fn new() -> Self {
        Self {
            distance: 100.0,
            faulty: false,
        }
    }
}

impl Sensor for LidarSensor {
    fn name(&self) -> &str {
        "LIDAR"
    }

    fn read(&self) -> f32 {
        if self.faulty {
            // implement possible faults 
            9999.0 // Simulate invalid reading
        } else {
            self.distance
        }
    }

    fn is_faulty(&self) -> bool {
        self.faulty
    }

    fn inject_fault(&mut self) {
        self.faulty = true;
    }

    fn reset_fault(&mut self) {
        self.faulty = false;
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}