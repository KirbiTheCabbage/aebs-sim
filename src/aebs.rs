
// aebs.rs
use crate::traits::Sensor;
use std::collections::VecDeque;
use std::collections::HashMap;
use crate::sensors::LidarSensor;


pub struct AebsSystem {
    pub active: bool,
    pub brake_level: u8,
    pub fault_detected: bool,
    pub sensors: Vec<Box<dyn Sensor>>,
    pub sensor_data_history: HashMap<String, VecDeque<f32>>,
    pub max_history: usize,
}

impl AebsSystem {
    pub fn new() -> Self {
        Self {
            active: true,
            brake_level: 0,
            fault_detected: false,
            sensors: vec![],
            sensor_data_history: HashMap::new(),
            max_history: 100,
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


        for sensor in &mut self.sensors {
            if let Some(lidar) = sensor.as_any_mut().downcast_mut::<LidarSensor>() {
                lidar.distance = (lidar.distance - 0.5).max(0.0);
            }

            let name = sensor.name();
            let reading = sensor.read();

            let history = self
                .sensor_data_history
                .entry(name.to_string())
                .or_insert_with(|| VecDeque::with_capacity(self.max_history));

            if history.len() >= self.max_history {
                history.pop_front();
            }

            history.push_back(reading);

            if reading < min_distance {
                min_distance = reading;
            }

            if sensor.is_faulty() {
                fault = true;
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
