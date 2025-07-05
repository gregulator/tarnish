// Run with:
// cargo run --bin=wooftrim_main | tee wooftrim.dxf

use tarnish::air;
use tarnish::dxf;
use tarnish::geom;
use tarnish::utils;

fn main() -> std::io::Result<()> {
    let mut dxf_writer = dxf::DxfWriter::new();
    let extent = geom::Bounds2 {
        min: geom::Vec2 {
            x: -air::WOOFTRIM_OUTER_RADIUS,
            y: -air::WOOFTRIM_OUTER_RADIUS,
        },
        max: geom::Vec2 {
            x: air::WOOFTRIM_OUTER_RADIUS,
            y: air::WOOFTRIM_OUTER_RADIUS,
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
            radius: air::WOOFTRIM_OUTER_RADIUS
        })
    );

    // CUTOUTS

    // BIG HOLE
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: utils::origin(),
            radius: air::WOOFTRIM_CUTOUT_RADIUS
        })
    );
    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: utils::origin(),
                    radius: air::WOOFER_BOLT_CIRCLE_RADIUS
                },
                num_holes: 8,
                hole_radius: air::WOOFTRIM_BOLT_HOLE_RADIUS,
                angle_offset: 22.5,
            }
        )
    );
    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);
    Ok(())
}
