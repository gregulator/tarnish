// Run with:
// cargo run --bin=supercube_a_frontpanel_main | tee supercube_a_frontpanel.dxf

// TODO: Figure out how grill cloth will be mounted.

use tarnish::projects::supercube;
use tarnish::dxf;
use tarnish::geom;
use tarnish::utils;

const PI: f64 = std::f64::consts::PI;

fn main() -> std::io::Result<()> {
    let mut dxf_writer = dxf::DxfWriter::new();
    let extent = geom::Bounds2 {
        min: geom::Vec2 {
            x: -supercube::OUTER_WIDTH/2.0,
            y: -supercube::OUTER_HEIGHT / 2.0
        },
        max: geom::Vec2 {
            x: supercube::OUTER_WIDTH/2.0,
            y: supercube::OUTER_HEIGHT / 2.0
        },
    };
    println!("{}", dxf::gen_header(extent));
    println!("{}", dxf::TABLES);
    println!("{}", dxf::BLOCKS);
    println!("{}", dxf::ENTITIES_HEADER);

    // OUTLINE SHAPE
    println!(
        "{}",
        utils::gen_rounded_rect(
            &mut dxf_writer,
            &utils::RoundedRect {
                ll: geom::Vec2 {
                    x: -supercube::OUTER_WIDTH / 2.0 - supercube::VENEER_WIDTH,
                    y: -supercube::OUTER_HEIGHT / 2.0 - supercube::VENEER_WIDTH
                },
                ur: geom::Vec2 {
                    x: supercube::OUTER_WIDTH / 2.0 + supercube::VENEER_WIDTH,
                    y: supercube::OUTER_HEIGHT / 2.0 + supercube::VENEER_WIDTH
                },
                round_radius: supercube::OUTER_ROUNDING
            }
        )
    );

    // CUTOUTS
    // Cavity
    println!(
        "{}",
        utils::gen_rounded_rect(
            &mut dxf_writer,
            &utils::RoundedRect {
                ll: geom::Vec2 {
                    x: -supercube::FRONT_PANEL_CAVITY_WIDTH / 2.0,
                    y: -supercube::FRONT_PANEL_CAVITY_HEIGHT / 2.0
                },
                ur: geom::Vec2 {
                    x: supercube::FRONT_PANEL_CAVITY_WIDTH / 2.0,
                    y: supercube::FRONT_PANEL_CAVITY_HEIGHT / 2.0
                },
                round_radius: supercube::CAVITY_ROUNDING
            }
        )
    );

    /*
    // checking CUTOUTS
    println!(
        "{}",
        utils::gen_rounded_rect(
            &mut dxf_writer,
            &utils::RoundedRect {
                ll: geom::Vec2 {
                    x: -supercube::CAVITY_WIDTH / 2.0,
                    y: -supercube::CAVITY_HEIGHT / 2.0
                },
                ur: geom::Vec2 {
                    x: supercube::CAVITY_WIDTH / 2.0,
                    y: supercube::CAVITY_HEIGHT / 2.0
                },
                round_radius: supercube::CAVITY_ROUNDING
            }
        )
    );
    // checking OUTLINE SHAPE
    println!(
        "{}",
        utils::gen_rounded_rect(
            &mut dxf_writer,
            &utils::RoundedRect {
                ll: geom::Vec2 {
                    x: -supercube::OUTER_WIDTH / 2.0,
                    y: -supercube::OUTER_HEIGHT / 2.0
                },
                ur: geom::Vec2 {
                    x: supercube::OUTER_WIDTH / 2.0,
                    y: supercube::OUTER_HEIGHT / 2.0
                },
                round_radius: supercube::OUTER_ROUNDING
            }
        )
    );
    */

    // Screw holes - should be countersunk
    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                  center: utils::origin(),
                  radius: supercube::FRONT_PANEL_BOLT_CIRCLE_RADIUS,
                },
                num_holes: 4,
                hole_radius: 3.0,
                angle_offset: 45.0,
            }
        )
    );

    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);
    Ok(())
}
