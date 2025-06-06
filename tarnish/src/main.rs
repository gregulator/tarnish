use serde::Serialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize)]
struct Vec2 {
    x: f64,
    y: f64,
}

#[derive(Serialize)]
struct Line {
    point: Vec2,
    norm: Vec2,
}

#[derive(Serialize)]
struct Rect {
    ll: Vec2, // Lower left.
    ur: Vec2, // Upper right.
}

#[derive(Serialize)]
struct Circle {
    center: Vec2,
    radius: f64,
}

#[derive(Serialize)]
struct Polyline {
    v: Vec<Vec2>,
}

#[derive(Serialize)]
#[allow(dead_code)]
enum Shape {
    Rect(Rect),
    Circle(Circle),
    Polyline(Polyline),
}

#[derive(Serialize)]
struct Part {
    outline: Shape,
    cutouts: Vec<Shape>,
    bendlines: Vec<Line>,
}

fn main() -> std::io::Result<()> {
    let p = Part {
        outline: Shape::Rect(Rect {
            ll: Vec2 { x: 0.0, y: 0.0 },
            ur: Vec2 { x: 100.0, y: 100.0 },
        }),
        cutouts: Vec::new(),
        bendlines: Vec::new(),
    };
    let serialized = serde_json::to_string(&p).unwrap();
    let mut file = File::create("out.json")?;
    write!(file, "{}", serialized)?;
    println!("serialized = {}", serialized);
    Ok(())
}
