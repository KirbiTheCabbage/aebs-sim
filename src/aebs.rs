// aebs.rs

// aebs.rs
use crate::traits::Sensor;

pub struct AebsSystem {
    pub active: bool,
    pub brake_level: u8,
    pub fault_detected: bool,
    pub sensors: Vec<Box<dyn Sensor>>,
}

impl AebsSystem {
    pub fn new() -> Self {
        Self {
            active: true,
            brake_level: 0,
            fault_detected: false,
            sensors: Vec::new(),
        }
    }

    pub fn add_sensor(&mut self, sensor: Box<dyn Sensor>) {
        self.sensors.push(sensor);
    }

    pub fn evaluate(&mut self) {
        if !self.active {
            self.brake_level = 0;
            return;
        }

        let mut fault = false;
        let mut min_distance = f32::MAX;

        for sensor in self.sensors.iter() {
            if sensor.is_faulty() {
                fault = true;
                continue;
            }

            let reading = sensor.read();
            if reading < min_distance {
                min_distance = reading;
            }
        }

        self.fault_detected = fault;

        self.brake_level = if fault {
            100
        } else if min_distance < 10.0 {
            100
        } else if min_distance < 25.0 {
            50
        } else {
            0
        };
    }
}
