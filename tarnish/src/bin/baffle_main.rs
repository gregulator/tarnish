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

// use std::fs::File;
// use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let bottom_gap = 74.0; // !
    let sub_outer_diameter = 384.0; // !
    let sub_bolt_diameter = 372.0; // !
    let sub_cutout_diameter = 350.0; // !
    let sub_sub_gap = 12.0; // !
    let sub_fr_gap = 12.0; // !
    let fr_bolt_diameter = 84.1; // !
    let fr_cutout_diameter = 64.0; // !
    let fr_outer_length = 70.0; // !
    let fr_fr_gap = 6.0; // !
    let top_gap = 74.0; // !

    let mut y: f64 = bottom_gap;

    let baffle_width = 762.0;
    let baffle_center_width = 434.0;
    let baffle_height = bottom_gap + sub_outer_diameter*2.0 + sub_sub_gap + sub_fr_gap + fr_outer_length*2.0 + fr_fr_gap + top_gap;

    let mut dxf_writer = dxf::DxfWriter::new();
    //let p = Part {
    //    outline: Shape::Rect(Rect {
    //        ll: Vec2 { x: 0.0, y: 0.0 },
    //        ur: Vec2 { x: 100.0, y: 100.0 },
    //    }),
    //    cutouts: Vec::new(),
    //    bendlines: Vec::new(),
    //};
    //let serialized = serde_json::to_string(&p).unwrap();
    // let mut file = File::create("out.dxf")?;
    // write!(file, "{}{}", dxf::HEADER, serialized)?;
    //println!("serialized = {}", serialized);
    let extent = geom::Bounds2 {
        min: geom::Vec2 { x: 0.0, y: 0.0 },
        max: geom::Vec2 {
            x: baffle_width,
            y: baffle_height,
        },
    };
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
                    point: geom::Vec2 { x: 0.0, y: 0.0 },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 { x: 0.0, y: 400.0 },
                    bulge: Some(0.05)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 150.0-5.0,
                        y: baffle_height-5.0,
                    },
                    bulge: Some(-0.2),
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 150.0,
                        y: baffle_height,
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 612.0,
                        y: baffle_height,
                    },
                    bulge: Some(0.05)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 { x: 760.0, y: 400.0 },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 { x: 760.0, y: 0.0 },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 762.0 - 24.0,
                        y: 0.0
                    },
                    bulge: Some(0.5)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 591.0 + 24.0,
                        y: 0.0
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 591.0 - 24.0,
                        y: 0.0
                    },
                    bulge: Some(0.25)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 171.0 + 24.0,
                        y: 0.0
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 171.0 - 24.0,
                        y: 0.0
                    },
                    bulge: Some(0.5)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 { x: 24.0, y: 0.0 },
                    bulge: None
                },
            ],
        })
    );

    // LOWER SUB
    y += sub_outer_diameter/2.0;
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: baffle_width/2.0,
                y: y,
            },
            radius: sub_cutout_diameter/2.0,
        })
    );
    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: baffle_width/2.0,
                        y: y
                    },
                    radius: sub_bolt_diameter/2.0,
                },
                num_holes: 8,
                hole_radius: 3.0,
                angle_offset: 22.5,
            }
        )
    );

    // UPPER SUB
    y += sub_outer_diameter + sub_sub_gap;

    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: baffle_width/2.0,
                y: y,
            },
            radius: sub_cutout_diameter/2.0,
        })
    );
    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: baffle_width/2.0,
                        y: y
                    },
                    radius: sub_bolt_diameter/2.0,
                },
                num_holes: 8,
                hole_radius: 3.0,
                angle_offset: 0.0,
            }
        )
    );

    // LOWER FULLRANGE
    y += sub_outer_diameter/2.0 + sub_fr_gap + fr_outer_length/2.0;
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: baffle_width/2.0,
                y: y
            },
            radius: fr_cutout_diameter/2.0,
        })
    );

    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: baffle_width/2.0,
                        y: y
                    },
                    radius: fr_bolt_diameter/2.0,
                },
                num_holes: 4,
                hole_radius: 3.0,
                angle_offset: 45.0,
            }
        )
    );


    // UPPER FULLRANGE
    y += fr_fr_gap + fr_outer_length;
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: baffle_width/2.0,
                y: y
            },
            radius: fr_cutout_diameter/2.0,
        })
    );

    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: baffle_width/2.0,
                        y: y
                    },
                    radius: fr_bolt_diameter/2.0,
                },
                num_holes: 4,
                hole_radius: 3.0,
                angle_offset: 45.0,
            }
        )
    );

    // BENDLINES
    let bend_0_x = baffle_width/2.0 - baffle_center_width/2.0;
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: geom::Vec2 { x: bend_0_x, y: 0.0 },
            p1: geom::Vec2 {
                x: bend_0_x,
                y: baffle_height,
            },
        })
    );
    let bend_1_x = baffle_width/2.0 + baffle_center_width/2.0;
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: geom::Vec2 { x: bend_1_x, y: 0.0 },
            p1: geom::Vec2 {
                x: bend_1_x,
                y: baffle_height,
            },
        })
    );
    //println!("{}", dxf::gen_circle(geom::Circle{
    //  center: geom::Vec2{x: 0.0, y: 0.0},
    //  radius: 100.0,
    //}));
    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);
    //println!("{}", dxf::gen_header(extent));
    //println!("{}", dxf::EMPTY2);

    Ok(())
}
