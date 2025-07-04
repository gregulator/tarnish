use crate::dxf;
use crate::geom;

const PI: f64 = std::f64::consts::PI;

pub fn origin() -> geom::Vec2 {
    return geom::Vec2 { x: 0.0, y: 0.0 };
}

pub struct BoltCircle {
    pub ring_circle: geom::Circle,
    pub num_holes: i32,
    pub hole_radius: f64,
    pub angle_offset: f64,
}

pub fn gen_bolt_circle(dxf_writer: &mut dxf::DxfWriter, bc: BoltCircle) -> std::string::String {
    let mut out = std::string::String::new();
    for i in 0..bc.num_holes {
        let angle =
            (bc.angle_offset * PI / 180.0) + (((i as f64) * 2.0 * PI) / (bc.num_holes as f64));
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

pub struct RoundedRect {
    pub ll: geom::Vec2,
    pub ur: geom::Vec2,
    pub round_radius: f64,
}

pub fn gen_rounded_rect(
    dxf_writer: &mut dxf::DxfWriter,
    rounded_rect: &RoundedRect,
) -> std::string::String {
    let ll = &rounded_rect.ll;
    let ur = &rounded_rect.ur;
    let ul = geom::Vec2 { x: ll.x, y: ur.y };
    let lr = geom::Vec2 { x: ur.x, y: ll.y };
    let round_radius = &rounded_rect.round_radius;
    let bulge = -0.42;
    return dxf_writer.gen_polyline(geom::Polyline {
        v: vec![
            geom::PolylineVertex {
                point: geom::Vec2 {
                    x: ll.x,
                    y: ll.y + round_radius,
                },
                bulge: None,
            },
            geom::PolylineVertex {
                point: geom::Vec2 {
                    x: ul.x,
                    y: ul.y - round_radius,
                },
                bulge: Some(bulge),
            },
            geom::PolylineVertex {
                point: geom::Vec2 {
                    x: ul.x + round_radius,
                    y: ul.y,
                },
                bulge: None,
            },
            geom::PolylineVertex {
                point: geom::Vec2 {
                    x: ur.x - round_radius,
                    y: ur.y,
                },
                bulge: Some(bulge),
            },
            geom::PolylineVertex {
                point: geom::Vec2 {
                    x: ur.x,
                    y: ur.y - round_radius,
                },
                bulge: None,
            },
            geom::PolylineVertex {
                point: geom::Vec2 {
                    x: lr.x,
                    y: lr.y + round_radius,
                },
                bulge: Some(bulge),
            },
            geom::PolylineVertex {
                point: geom::Vec2 {
                    x: lr.x - round_radius,
                    y: lr.y,
                },
                bulge: None,
            },
            geom::PolylineVertex {
                point: geom::Vec2 {
                    x: ll.x + round_radius,
                    y: lr.y,
                },
                bulge: Some(bulge),
            },
        ],
    });
}

pub struct Pill {
    pub start: geom::Vec2,
    pub end: geom::Vec2,
    pub thickness: f64,
}

pub fn gen_pill(dxf_writer: &mut dxf::DxfWriter, pill: &Pill) -> std::string::String {
    let prp = perp2(&pill.end, &pill.start);
    let ll = add(&pill.start, &scale(-pill.thickness / 2.0, &prp));
    let ul = add(&pill.start, &scale(pill.thickness / 2.0, &prp));
    let ur = add(&pill.end, &scale(pill.thickness / 2.0, &prp));
    let lr = add(&pill.end, &scale(-pill.thickness / 2.0, &prp));

    return dxf_writer.gen_polyline(geom::Polyline {
        v: vec![
            geom::PolylineVertex {
                point: ll,
                bulge: Some(-1.0),
            },
            geom::PolylineVertex {
                point: ul,
                bulge: None,
            },
            geom::PolylineVertex {
                point: ur,
                bulge: Some(-1.0),
            },
            geom::PolylineVertex {
                point: lr,
                bulge: None,
            },
        ],
    });
}

pub struct TerminalHoleWithNotch {
    pub hole_circle: geom::Circle,
    pub notch_length: f64,
}

pub fn gen_terminal_hole_with_notch(
    dxf_writer: &mut dxf::DxfWriter,
    thwn: &TerminalHoleWithNotch,
) -> std::string::String {
    let start = geom::Vec2 {
        x: thwn.hole_circle.center.x - thwn.notch_length,
        y: thwn.hole_circle.center.y - thwn.hole_circle.radius,
    };
    return dxf_writer.gen_polyline(geom::Polyline {
        v: vec![
            geom::PolylineVertex {
                point: copy(&start),
                bulge: None,
            },
            geom::PolylineVertex {
                point: add(
                    &start,
                    &geom::Vec2 {
                        x: 0.0,
                        y: thwn.notch_length,
                    },
                ),
                bulge: None,
            },
            geom::PolylineVertex {
                point: add(
                    &start,
                    &geom::Vec2 {
                        x: thwn.notch_length,
                        y: thwn.notch_length,
                    },
                ),
                bulge: None,
            },
            geom::PolylineVertex {
                point: add(
                    &start,
                    &geom::Vec2 {
                        x: thwn.notch_length,
                        y: 0.0,
                    },
                ),
                bulge: Some(-2.0 * thwn.hole_circle.radius / thwn.notch_length),
            },
        ],
    });
}

pub fn perp(v: &geom::Vec2) -> geom::Vec2 {
    return normalize(&geom::Vec2 { x: v.y, y: -v.x });
}

pub fn perp2(v0: &geom::Vec2, v1: &geom::Vec2) -> geom::Vec2 {
    return perp(&geom::Vec2 {
        x: v1.x - v0.x,
        y: v1.y - v0.y,
    });
}

pub fn add(v0: &geom::Vec2, v1: &geom::Vec2) -> geom::Vec2 {
    return geom::Vec2 {
        x: v0.x + v1.x,
        y: v0.y + v1.y,
    };
}

pub fn sub(v0: &geom::Vec2, v1: &geom::Vec2) -> geom::Vec2 {
    return geom::Vec2 {
        x: v0.x - v1.x,
        y: v0.y - v1.y,
    };
}

pub fn scale(s: f64, v: &geom::Vec2) -> geom::Vec2 {
    return geom::Vec2 {
        x: s * v.x,
        y: s * v.y,
    };
}

// Returns v0 + s*v1
pub fn madd(v0: &geom::Vec2, v1: &geom::Vec2, s: f64) -> geom::Vec2 {
    return geom::Vec2 {
        x: v0.x + s * v1.x,
        y: v0.y + s * v1.y,
    };
}

pub fn copy(v: &geom::Vec2) -> geom::Vec2 {
    return geom::Vec2 { x: v.x, y: v.y };
}

pub fn mirror_x(v: &geom::Vec2) -> geom::Vec2 {
    return geom::Vec2 { x: -v.x, y: v.y };
}

pub fn norm(v: &geom::Vec2) -> f64 {
    return f64::sqrt(v.x * v.x + v.y * v.y);
}

pub fn normalize(v: &geom::Vec2) -> geom::Vec2 {
    let n = norm(v);
    return geom::Vec2 {
        x: v.x / n,
        y: v.y / n,
    };
}

pub fn transform_point(
    origin: &geom::Vec2,
    x_axis: &geom::Vec2,
    y_axis: &geom::Vec2,
    point: &geom::Vec2,
) -> geom::Vec2 {
    return madd(&madd(origin, x_axis, point.x), y_axis, point.y);
}
