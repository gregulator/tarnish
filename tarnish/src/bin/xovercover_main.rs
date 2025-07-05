// Run with:
// cargo run --bin=baffle_main | tee baffle.dxf

// TODO:
// Baffle attachment holes
// Finalize all dimensions.
// Polycarbonate cover
use tarnish::air;
use tarnish::dxf;
use tarnish::geom;

// use std::fs::File;
// use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut dxf_writer = dxf::DxfWriter::new();
    let extent = geom::Bounds2 {
        min: geom::Vec2 { x: 0.0, y: 0.0 },
        max: geom::Vec2 {
            x: 1000.0,
            y: 1000.0,
        },
    };

    let rr = 24.0;

    println!("{}", dxf::gen_header(extent));
    println!("{}", dxf::TABLES);
    println!("{}", dxf::BLOCKS);
    println!("{}", dxf::ENTITIES_HEADER);
    println!(
        "{}",
        dxf_writer.gen_polyline(geom::Polyline {
            v: vec![
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: -air::XOVERCOVER_BOTTOM_WIDTH / 2.0,
                        y: -air::XOVERCOVER_HEIGHT / 2.0 + rr
                    },
                    bulge: Some(0.15)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: -air::XOVERCOVER_TOP_WIDTH / 2.0,
                        y: air::XOVERCOVER_HEIGHT / 2.0 - rr
                    },
                    bulge: Some(-0.4)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: -air::XOVERCOVER_TOP_WIDTH / 2.0 + rr,
                        y: air::XOVERCOVER_HEIGHT / 2.0
                    },
                    bulge: Some(0.10)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::XOVERCOVER_TOP_WIDTH / 2.0 - rr,
                        y: air::XOVERCOVER_HEIGHT / 2.0
                    },
                    bulge: Some(-0.4)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::XOVERCOVER_TOP_WIDTH / 2.0,
                        y: air::XOVERCOVER_HEIGHT / 2.0 - rr
                    },
                    bulge: Some(0.15)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::XOVERCOVER_BOTTOM_WIDTH / 2.0,
                        y: -air::XOVERCOVER_HEIGHT / 2.0 + rr
                    },
                    bulge: Some(-0.6)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::XOVERCOVER_BOTTOM_WIDTH / 2.0 - rr,
                        y: -air::XOVERCOVER_HEIGHT / 2.0
                    },
                    bulge: Some(0.10)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: -air::XOVERCOVER_BOTTOM_WIDTH / 2.0 + rr,
                        y: -air::XOVERCOVER_HEIGHT / 2.0
                    },
                    bulge: Some(-0.6)
                },
            ],
        })
    );

    // CUTOUTS

    // stud cutouts for crossover board
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: -air::STAND_XOVER_HOLE_SPACING_X / 2.0,
                y: -air::STAND_XOVER_HOLE_SPACING_Y / 2.0,
            },
            radius: air::STAND_XOVER_HOLE_RADIUS
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: -air::STAND_XOVER_HOLE_SPACING_X / 2.0,
                y: air::STAND_XOVER_HOLE_SPACING_Y / 2.0,
            },
            radius: air::STAND_XOVER_HOLE_RADIUS
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: air::STAND_XOVER_HOLE_SPACING_X / 2.0,
                y: -air::STAND_XOVER_HOLE_SPACING_Y / 2.0,
            },
            radius: air::STAND_XOVER_HOLE_RADIUS
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: air::STAND_XOVER_HOLE_SPACING_X / 2.0,
                y: air::STAND_XOVER_HOLE_SPACING_Y / 2.0,
            },
            radius: air::STAND_XOVER_HOLE_RADIUS
        })
    );

    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);

    Ok(())
}
