use std::f32::consts::PI;

#[derive(Debug)]
struct Rectangle {
    w: f32,
    h: f32,
}

impl Rectangle {
    fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
}

#[derive(Debug)]
struct Circle {
    r: f32,
}

impl Circle {
    fn new(r: f32) -> Self {
        Self { r }
    }
}

trait Shape {
    fn area(&self) -> f32;
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.r.powi(2) * PI
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.w * self.h
    }
}

fn main() {
    let rect = Rectangle::new(3.3, 4.4);
    let circ = Circle::new(1.0);

    let shapes: Vec<&dyn Shape> = vec![&rect, &circ];

    println!("{:?} has area {}", rect, rect.area());
    println!("{:?} has area {}", circ, circ.area());

    for shape in shapes {
        println!("Shape has area {}", shape.area());
    }
}
