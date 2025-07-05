// Run with:
// cargo run --bin=woofgasket_main | tee wooftrim.dxf

use tarnish::air;
use tarnish::dxf;
use tarnish::geom;
use tarnish::utils;

fn main() -> std::io::Result<()> {
    let mut dxf_writer = dxf::DxfWriter::new();
    let extent = geom::Bounds2 {
        min: geom::Vec2 {
            x: -air::WOOFGASKET_OUTER_RADIUS,
            y: -air::WOOFGASKET_OUTER_RADIUS,
        },
        max: geom::Vec2 {
            x: air::WOOFGASKET_OUTER_RADIUS,
            y: air::WOOFGASKET_OUTER_RADIUS,
        },
    };
    println!("{}", dxf::gen_header(extent));
    println!("{}", dxf::TABLES);
    println!("{}", dxf::BLOCKS);
    println!("{}", dxf::ENTITIES_HEADER);

    // OUTLINE SHAPE
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: utils::origin(),
            radius: air::WOOFGASKET_OUTER_RADIUS
        })
    );

    // CUTOUTS

    // STAND MOUNTING HOLES
    // Lower
    println!(
        "{}",
        utils::gen_pill(
            &mut dxf_writer,
            &utils::Pill {
                start: geom::Vec2 {
                    x: air::BAFFLE_LL_STAND_HOLE_CENTER_X - air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_LOWER_STAND_HOLE_CENTER_Y
                },
                end: geom::Vec2 {
                    x: air::BAFFLE_LL_STAND_HOLE_CENTER_X + air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_LOWER_STAND_HOLE_CENTER_Y
                },
                thickness: air::BAFFLE_STAND_HOLE_THICKNESS,
            }
        )
    );
    // Upper
    println!(
        "{}",
        utils::gen_pill(
            &mut dxf_writer,
            &utils::Pill {
                start: geom::Vec2 {
                    x: air::BAFFLE_UL_STAND_HOLE_CENTER_X - air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_UPPER_STAND_HOLE_CENTER_Y
                },
                end: geom::Vec2 {
                    x: air::BAFFLE_UL_STAND_HOLE_CENTER_X + air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_UPPER_STAND_HOLE_CENTER_Y
                },
                thickness: air::BAFFLE_STAND_HOLE_THICKNESS,
            }
        )
    );
    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);
    Ok(())
}
