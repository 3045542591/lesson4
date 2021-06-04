fn main() {
    let light = TrafficLight::Green.traffic_light_time();
    println!("交通登亮{}秒", light);
}

pub trait OutputTime{
    fn traffic_light_time(&self) -> u8;
}



pub enum TrafficLight {
    Red,
    Yellow,
    Green, 
}


impl OutputTime for TrafficLight {
    fn traffic_light_time(&self) -> u8 {
        match self {
            TrafficLight::Red => 20,
            TrafficLight::Green => 30,
            TrafficLight::Yellow => 10,
        }
    }
}