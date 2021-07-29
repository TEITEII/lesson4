pub fn main() {
    let rectangle = Rectangle { length:6.5,width:7.8 };
    let _triangle = Triangle { length:8.4,width:9.6 };
    let r= &rectangle.area();
    println!("The area of form is : {} ",r);
}

pub trait Area {
    fn area (&self) -> f32;
}

pub struct Rectangle {
    length:f32,
    width:f32,
}

#[allow(dead_code)]
pub struct  Triangle {
    length:f32,
    width:f32,
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.length*self.width  
    }
}

pub fn area<T:Area> (form:&T) -> f32 { form.area() }

