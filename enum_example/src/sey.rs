struct Circle{
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64{
        return std::f64::consts::PI * (self.radius * self.radius)
    }
}

pub fn get_circle_area(x: f64, y: f64, radius: f64) -> f64 {
    let circle = Circle{x: x, y: y, radius: radius};
    return circle.area();
}

pub fn sey(name: String) -> String{
    return name
}

pub fn unsafe_code(){
    let x = 2;
    assert!(x == 3);    
}