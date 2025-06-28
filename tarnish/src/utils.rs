use crate::dxf;
use crate::geom;

const PI: f64 = std::f64::consts::PI;

pub struct BoltCircle {
    pub ring_circle: geom::Circle,
    pub num_holes: i32,
    pub hole_radius: f64,
    pub angle_offset: f64,
}

pub fn gen_bolt_circle(dxf_writer: &mut dxf::DxfWriter, bc: BoltCircle) -> std::string::String {
    let mut out = std::string::String::new();
    for i in 0..bc.num_holes {
        let angle = (bc.angle_offset*PI/180.0) + (((i as f64) * 2.0 * PI) / (bc.num_holes as f64));
        let hole_center = geom::Vec2 {
            x: bc.ring_circle.center.x + bc.ring_circle.radius * f64::cos(angle),
            y: bc.ring_circle.center.y + bc.ring_circle.radius * f64::sin(angle),
        };
        let hole_circle = geom::Circle {
            center: hole_center,
            radius: bc.hole_radius,
        };
        out += &dxf_writer.gen_circle(hole_circle);
        if i != bc.num_holes - 1 {
            out += "\n";
        }
    }
    out.to_string()
}
