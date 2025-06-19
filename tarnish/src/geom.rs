use serde::Serialize;

#[derive(Serialize)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Serialize)]
pub struct Bounds2 {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Serialize)]
pub struct Line {
    pub point: Vec2,
    pub norm: Vec2,
}

#[derive(Serialize)]
pub struct LineSeg {
    pub p0: Vec2,
    pub p1: Vec2,
}

#[derive(Serialize)]
pub struct Rect {
    pub ll: Vec2, // Lower left.
    pub ur: Vec2, // Upper right.
}

#[derive(Serialize)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f64,
}

#[derive(Serialize)]
pub struct Polyline {
    pub v: Vec<Vec2>,
}

#[derive(Serialize)]
#[allow(dead_code)]
pub enum Shape {
    Rect(Rect),
    Circle(Circle),
    Polyline(Polyline),
}

#[derive(Serialize)]
pub struct Part {
    pub outline: Shape,
    pub cutouts: Vec<Shape>,
    pub bendlines: Vec<Line>,
}
