// Run with:
// cargo run --bin=baffle_main | tee baffle.dxf

// TODO:
// - Set units to MM (Done but SCS still checks)
// - Rounded rectangles
// - Bolt hole rings
// - Gen handles (not really necessary)
// - Polygons
// - Splines
use tarnish::dxf;
use tarnish::geom;

const PI: f64 = std::f64::consts::PI;

// use std::fs::File;
// use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let stand_depth = 40.0;
    let thickness = stand_depth;
    let wing_length = 100.0;
    let middle_length = 400.0;
    let bend_offset = 50.0;
    let baffle_side_angle = 60.0 * PI / 180.0;

    let spine0 = geom::Vec2 { x: 0.0, y: 0.0 };
    let spine1 = geom::Vec2 {
        x: wing_length * f64::cos(baffle_side_angle),
        y: wing_length * f64::sin(baffle_side_angle),
    };
    let spine2 = geom::Vec2 {
        x: spine1.x + middle_length,
        y: spine1.y,
    };
    let spine3 = geom::Vec2 {
        x: spine2.x + middle_length + wing_length * f64::cos(baffle_side_angle),
        y: spine2.y - middle_length + wing_length * f64::sin(baffle_side_angle),
    };

    let mut dxf_writer = dxf::DxfWriter::new();
    let extent = geom::Bounds2 {
        min: geom::Vec2 { x: 0.0, y: 0.0 },
        max: geom::Vec2 {
            x: 1000.0,
            y: 1000.0,
        },
    };

    let s0_a = utils::add(
        spine0,
        utils::scale(0.5 * thickness, utils::perp2(spine1, spine0)),
    );
    let s1_a = utils::add(
        spine1,
        utils::scale(0.5 * thickness, utils::perp2(spine2, spine1)),
    );
    let s2_a = utils::add(
        spine2,
        utils::scale(0.5 * thickness, utils::perp2(spine2, spine1)),
    );
    let s3_a = utils::add(
        spine2,
        utils::scale(0.5 * thickness, utils::perp2(spine3, spine2)),
    );

    let s0_a = utils::add(
        spine0,
        utils::scale(-0.5 * thickness, utils::perp2(spine1, spine0)),
    );
    let s1_a = utils::add(
        spine1,
        utils::scale(-0.5 * thickness, utils::perp2(spine2, spine1)),
    );
    let s2_a = utils::add(
        spine2,
        utils::scale(-0.5 * thickness, utils::perp2(spine2, spine1)),
    );
    let s3_a = utils::add(
        spine2,
        utils::scale(-0.5 * thickness, utils::perp2(spine3, spine2)),
    );

    println!("{}", dxf::gen_header(extent));
    println!("{}", dxf::TABLES);
    println!("{}", dxf::BLOCKS);
    println!("{}", dxf::ENTITIES_HEADER);
    // CHECK ALL OF THIS BEFORE ORDERING!
    println!(
        "{}",
        dxf_writer.gen_polyline(geom::Polyline {
            v: vec![
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 0.0,
                        y: f64::sin(baffle_side_angle)
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 0.0,
                        y: stand_depth
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: wing_length,
                        y: stand_depth
                    },
                    bulge: Some(-1.0)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: wing_length + middle_length,
                        y: stand_depth
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: wing_length * 2.0 + middle_length,
                        y: stand_depth
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: wing_length * 2.0 + middle_length,
                        y: 0.0
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: wing_length + middle_length - stand_depth,
                        y: 0.0
                    },
                    bulge: Some(1.25)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: wing_length + stand_depth,
                        y: 0.0
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 { x: 0.0, y: 0.0 },
                    bulge: None
                },
            ],
        })
    );

    // BENDLINES
    let bend_0_x = bend_offset;
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: geom::Vec2 {
                x: bend_0_x,
                y: 0.0
            },
            p1: geom::Vec2 {
                x: bend_0_x * 1.5,
                y: 40.0,
            },
        })
    );
    let bend_1_x = wing_length * 2.0 + middle_length - bend_offset;
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: geom::Vec2 {
                x: bend_1_x - 0.5 * bend_offset,
                y: 0.0
            },
            p1: geom::Vec2 {
                x: bend_1_x,
                y: 40.0,
            },
        })
    );
    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);

    Ok(())
}
