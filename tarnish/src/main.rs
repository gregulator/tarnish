#[allow(dead_code)]
#[derive(Debug)]
struct Vec2 {
  x: f64,
  y: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Line {
  point: Vec2,
  norm: Vec2,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rect {
  ll: Vec2,  // Lower left.
  ur: Vec2,  // Upper right.
}

#[allow(dead_code)]
#[derive(Debug)]
struct Circle {
  center: Vec2,
  radius: f64,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Polyline {
  v: Vec<Vec2>,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Shape {
  Rect(Rect),
  Circle(Circle),
  Polyline(Polyline),
}

#[allow(dead_code)]
#[derive(Debug)]
struct Part {
  outline: Shape,
  cutouts: Vec<Shape>,
  bendlines: Vec<Line>,
}

fn main() {
    let p = Part {
      outline: Shape::Rect(Rect{
        ll: Vec2{x: 0.0, y: 0.0},
        ur: Vec2{x: 100.0, y: 100.0},
      }),
      cutouts: Vec::new(),
      bendlines: Vec::new(),
    };
    println!("part {:#?}", p);
}
