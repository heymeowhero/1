enum Trafficlight{
    Red,
    Green,
    Yellow,
}trait LightTime{
    fn time(&self) -> u8;
}
impl LightTime for Trafficlight {
    fn time(&self) -> u8 {
        match self{
            Trafficlight::Red => 10,
            Trafficlight::Green => 20,
            Trafficlight::Yellow => 30,
        }
    }
}
fn main() {
    let light1 = Trafficlight::Red;
    println!("light1 is: {}", light1.time());
    let light2 = Trafficlight::Green;
    println!("light2 is: {}", light2.time());
    let light3 = Trafficlight::Yellow;
    println!("light3 is: {}", light3.time());
}


