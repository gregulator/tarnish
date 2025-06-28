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
use tarnish::utils;

const PI: f64 = std::f64::consts::PI;

// use std::fs::File;
// use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let stand_depth = 40.0;
    let thickness = stand_depth;
    let wing_length = 200.0;
    let middle_length = 400.0;
    let bend_offset = 150.0;
    let baffle_side_angle = 60.0*PI/180.0;
    let side_angle = PI/2.0-baffle_side_angle;


    let spine0 = geom::Vec2{x: 0.0, y: 0.0};
    let spine1 = geom::Vec2{x: wing_length*f64::cos(side_angle),
                            y: wing_length*f64::sin(side_angle)};
    let spine2 = geom::Vec2{x: spine1.x + middle_length,
                            y: spine1.y};
    let spine3 = geom::Vec2{x: spine2.x + wing_length*f64::cos(side_angle),
                            y: 0.0};

    let mut dxf_writer = dxf::DxfWriter::new();
    let extent = geom::Bounds2 {
        min: geom::Vec2 { x: 0.0, y: 0.0 },
        max: geom::Vec2 {
            x: 1000.0,
            y: 1000.0,
        },
    };

    //let s0_a = utils::add(&spine0, &utils::scale(0.5*thickness, &utils::perp2(&spine1, &spine0)));
    let s0_a = utils::add(&spine0, &utils::scale(0.5*thickness, &utils::perp2(&spine1, &spine0)));
    let s1_a = utils::add(&spine1, &utils::scale(0.5*thickness, &utils::perp2(&spine1, &spine0)));
    let s2_a = utils::add(&spine2, &utils::scale(0.5*thickness, &utils::perp2(&spine3, &spine2)));
    let s3_a = utils::add(&spine3, &utils::scale(0.5*thickness, &utils::perp2(&spine3, &spine2)));

    let s0_b = utils::add(&spine0, &utils::scale(-0.5*thickness, &utils::perp2(&spine1, &spine0)));
    let s1_b = utils::add(&spine1, &utils::scale(-0.5*thickness, &utils::perp2(&spine1, &spine0)));
    let s2_b = utils::add(&spine2, &utils::scale(-0.5*thickness, &utils::perp2(&spine3, &spine2)));
    let s3_b = utils::add(&spine3, &utils::scale(-0.5*thickness, &utils::perp2(&spine3, &spine2)));

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
                    point: utils::copy(&s0_a),
                    bulge: None
                },
                geom::PolylineVertex {
                    point: utils::copy(&s1_a),
                    bulge: Some(-1.0),
                },
                geom::PolylineVertex {
                    point: utils::copy(&s2_a),
                    bulge: None
                },
                geom::PolylineVertex {
                    point: utils::copy(&s3_a),
                    bulge: None
                },
                geom::PolylineVertex {
                    point: utils::copy(&s3_b),
                    bulge: None
                },
                geom::PolylineVertex {
                    point: s2_b,
                    bulge: None
                },
                geom::PolylineVertex {
                    point: s1_b,
                    bulge: None
                },
                geom::PolylineVertex {
                    point: utils::copy(&s0_b),
                    bulge: None
                },
                geom::PolylineVertex {
                    point: utils::copy(&s0_a),
                    bulge: None
                },
            ],
        })
    );

    // CUTOUTS
    println!(
        "{}",
        dxf_writer.gen_polyline(geom::Polyline {
            v: vec![
                geom::PolylineVertex {
                    point: utils::add(&s1_a, &geom::Vec2{x:thickness, y:0.0}),
                    bulge: Some(-1.0),
                },
                geom::PolylineVertex {
                    point: utils::add(&s2_a, &geom::Vec2{x:-thickness, y:0.0}),
                    bulge: None,
                },
            ]
        })
    );


    // BENDLINES
    let bend0_a = utils::add(&s0_a, &utils::scale(bend_offset, &utils::normalize(&utils::sub(&spine1, &spine0))));
    let bend0_b = utils::add(&s0_b, &utils::scale(bend_offset, &utils::normalize(&utils::sub(&spine1, &spine0))));
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: bend0_a,
            p1: bend0_b,
        })
    );
    let bend0_a = utils::add(&s3_a, &utils::scale(-bend_offset, &utils::normalize(&utils::sub(&spine3, &spine2))));
    let bend0_b = utils::add(&s3_b, &utils::scale(-bend_offset, &utils::normalize(&utils::sub(&spine3, &spine2))));
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: bend0_a,
            p1: bend0_b,
        })
    );
    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);

    Ok(())
}
