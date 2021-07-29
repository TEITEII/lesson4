fn main() {
    let green = Lightcolors::Green;
    let _yellow = Lightcolors::Yellow;
    let _red = Lightcolors::Red;
    notify(&green);
    notify(&_yellow);
    notify(&_red);
}
pub trait Trafficlights {
    fn lightingtime(&self) -> u8;
}

pub enum Lightcolors {
    Green,
    Yellow,
    Red,
}

impl Trafficlights for Lightcolors {
    fn lightingtime(&self) -> u8 {
        match &self {
            Lightcolors::Green => 40,
            Lightcolors::Yellow => 5,
            Lightcolors::Red => 30,
        }
    }
}

pub fn notify<T:Trafficlights>(item:&T) {
    println!("This color of trafficlight lighting for {}s",item.lightingtime());
}