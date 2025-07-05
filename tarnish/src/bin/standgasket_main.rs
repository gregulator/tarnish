// Run with:
// cargo run --bin=baffle_main | tee baffle.dxf

use tarnish::air;
use tarnish::dxf;
use tarnish::geom;
use tarnish::utils;

fn main() -> std::io::Result<()> {
    let mut dxf_writer = dxf::DxfWriter::new();
    let extent = geom::Bounds2 {
        min: geom::Vec2 { x: 0.0, y: 0.0 },
        max: geom::Vec2 {
            x: 1000.0,
            y: 1000.0,
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
                    point: geom::Vec2 {
                        x: -air::STANDGASKET_WIDTH/2.0,
                        y: -air::STANDGASKET_HEIGHT/2.0
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: -air::STANDGASKET_WIDTH/2.0,
                        y: air::STANDGASKET_HEIGHT/2.0
                    },
                    bulge: Some(-1.0)
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::STANDGASKET_WIDTH/2.0,
                        y: air::STANDGASKET_HEIGHT / 2.0
                    },
                    bulge: None
                },
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::STANDGASKET_WIDTH/2.0,
                        y: -air::STANDGASKET_HEIGHT /2.0
                    },
                    bulge: None
                },
            ]
        })
    );

    // CUTOUTS
    // STAND MOUNTING HOLES
    // Lower-left
    println!(
        "{}",
        utils::gen_pill(
            &mut dxf_writer,
            &utils::Pill {
                start: geom::Vec2 {
                    x: air::BAFFLE_STAND_HOLE_OFFSET_X - air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_LOWER_STAND_HOLE_CENTER_Y - air::STANDGASKET_HEIGHT/2.0 - air::STANDGASKET_PADDING/2.0
                },
                end: geom::Vec2 {
                    x: air::BAFFLE_STAND_HOLE_OFFSET_X + air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_LOWER_STAND_HOLE_CENTER_Y - air::STANDGASKET_HEIGHT/2.0 - air::STANDGASKET_PADDING/2.0
                },
                thickness: air::BAFFLE_STAND_HOLE_THICKNESS,
            }
        )
    );
    // Upper-left
    println!(
        "{}",
        utils::gen_pill(
            &mut dxf_writer,
            &utils::Pill {
                start: geom::Vec2 {
                    x: -air::BAFFLE_STAND_HOLE_OFFSET_X - air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_UPPER_STAND_HOLE_CENTER_Y - air::STANDGASKET_HEIGHT/2.0 - air::STANDGASKET_PADDING/2.0
                },
                end: geom::Vec2 {
                    x: -air::BAFFLE_STAND_HOLE_OFFSET_X + air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_UPPER_STAND_HOLE_CENTER_Y - air::STANDGASKET_HEIGHT/2.0 - air::STANDGASKET_PADDING/2.0
                },
                thickness: air::BAFFLE_STAND_HOLE_THICKNESS,
            }
        )
    );

    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);

    Ok(())
}
