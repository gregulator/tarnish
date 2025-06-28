// Run with:
// cargo run --bin=baffle_main | tee baffle.dxf

// TODO:
// Baffle attachment holes
// Finalize all dimensions.
// Polycarbonate cover
use tarnish::dxf;
use tarnish::geom;
use tarnish::utils;

const PI: f64 = std::f64::consts::PI;

// use std::fs::File;
// use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let thickness = 40.0;
    let wing_length = 200.0;
    let middle_length = 400.0;
    let bend_offset = 150.0;
    let baffle_side_angle = 60.0*PI/180.0;
    let side_angle = PI/2.0-baffle_side_angle;
    let stand_depth = 400.0;
    let stand_back_width = 300.0;

    let spine0 = geom::Vec2{x: 0.0, y: 0.0};
    let spine1 = geom::Vec2{x: wing_length*f64::cos(side_angle),
                            y: wing_length*f64::sin(side_angle)};
    let spine2 = geom::Vec2{x: spine1.x + middle_length,
                            y: spine1.y};
    let spine3 = geom::Vec2{x: spine2.x + wing_length*f64::cos(side_angle),
                            y: 0.0};

    let total_middle_x = (spine1.x+spine2.x)/2.0;


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

    let b0 = geom::Vec2{x:s1_a.x+(s2_a.x-s1_a.x)/2.0 - stand_back_width/2.0, y:stand_depth};
    let b1 = geom::Vec2{x:s2_a.x-((s2_a.x-s1_a.x)/2.0 - stand_back_width/2.0), y:stand_depth};

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
                    bulge: Some(0.35)
                },

                geom::PolylineVertex {
                    point: b0,
                    bulge: Some(-0.25)
                },
                geom::PolylineVertex {
                    point: b1,
                    bulge: Some(0.35)
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
                    bulge: Some(0.15)
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
    // stud cutouts for crossover board
    let xover_hole_spacing_x = 170.0;
    let xover_hole_spacing_y = 117.5;
    let xover_hole_radius = 5.0;
    let xover_center_y = 200.0;

    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: total_middle_x-xover_hole_spacing_x/2.0,
                y: xover_center_y-xover_hole_spacing_y/2.0,
            },
            radius: xover_hole_radius
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: total_middle_x+xover_hole_spacing_x/2.0,
                y: xover_center_y-xover_hole_spacing_y/2.0,
            },
            radius: xover_hole_radius
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: total_middle_x+xover_hole_spacing_x/2.0,
                y: xover_center_y+xover_hole_spacing_y/2.0,
            },
            radius: xover_hole_radius
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: total_middle_x-xover_hole_spacing_x/2.0,
                y: xover_center_y+xover_hole_spacing_y/2.0,
            },
            radius: xover_hole_radius
        })
    );

    // terminal cutouts
    let terminal_spacing=50.0;
    let terminal_y=400.0;
    let terminal_radius=8.0;
    let terminal_notch=1.0;
    println!(
        "{}",
        utils::gen_terminal_hole_with_notch(&mut dxf_writer, &utils::TerminalHoleWithNotch {
            hole_circle: geom::Circle{
              center: geom::Vec2 {
                  x: (s2_a.x+s1_a.x)/2.0-terminal_spacing/2.0,
                  y: terminal_y 
              },
              radius: 8.0,
            },
            notch_length: 1.0,
        })
    );
    println!(
        "{}",
        utils::gen_terminal_hole_with_notch(&mut dxf_writer, &utils::TerminalHoleWithNotch {
            hole_circle: geom::Circle{
              center: geom::Vec2 {
                  x: (s2_a.x+s1_a.x)/2.0+terminal_spacing/2.0,
                  y: terminal_y 
              },
              radius: 8.0,
            },
            notch_length: 1.0,
        })
    );
    /*println!(
        "{}",
        dxf_writer.gen_polyline(geom::Polyline {
            v: vec![
                geom::PolylineVertex {
                    point: utils::add(&s1_a, &geom::Vec2{x:thickness, y:0.0}),
                    bulge: Some(-0.3),
                },
                geom::PolylineVertex {
                    point: utils::add(&s2_a, &geom::Vec2{x:-thickness, y:0.0}),
                    bulge: None,
                },
            ]
        })
    );*/


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
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: geom::Vec2{x:235.0, y:350.0},
            p1: geom::Vec2{x:610.0, y:350.0}
        })
    );
    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);

    Ok(())
}
