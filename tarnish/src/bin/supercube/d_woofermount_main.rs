// Run with:
// cargo run --bin=supercube_d_woofermount_main | tee supercube_d_woofermount.dxf

// TODO: Add pilot holes

use tarnish::projects::supercube;
use tarnish::dxf;
use tarnish::geom;
use tarnish::utils;

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

    // CUTOUTS
    // Woofer cutout
    println!(
        "{}",
        &dxf_writer.gen_circle(
            geom::Circle {
               center: utils::origin(),
               radius: supercube::WOOFER_CUTOUT_RADIUS,
            }
        )
    );

    // Top-right port
    println!(
        "{}",
        utils::gen_rounded_corner_rect(
            &mut dxf_writer,
            &utils::RoundedCornerRect {
                rect: geom::Rect{
                    ll: geom::Vec2 {
                        x: supercube::WOOFER_MOUNT_BRIDGE_THICKNESS/2.0,
                        y: supercube::WOOFER_MOUNT_BRIDGE_THICKNESS/2.0,
                    },
                    ur: geom::Vec2 {
                        x: supercube::CAVITY_WIDTH/2.0,
                        y: supercube::CAVITY_HEIGHT/2.0,
                    },
                },
                ll_round_radius: -supercube::TUBE_OUTER_RADIUS,
                ul_round_radius: 2.0,
                ur_round_radius: supercube::CAVITY_ROUNDING,
                lr_round_radius: 2.0,
            }
        )
    );

    // Bottom-right port
    println!(
        "{}",
        utils::gen_rounded_corner_rect(
            &mut dxf_writer,
            &utils::RoundedCornerRect {
                rect: geom::Rect{
                    ll: geom::Vec2 {
                        x: supercube::WOOFER_MOUNT_BRIDGE_THICKNESS/2.0,
                        y: -supercube::CAVITY_HEIGHT/2.0,
                    },
                    ur: geom::Vec2 {
                        x: supercube::CAVITY_WIDTH/2.0,
                        y: -supercube::WOOFER_MOUNT_BRIDGE_THICKNESS/2.0,
                    },
                },
                ll_round_radius: 2.0,
                ul_round_radius: -supercube::TUBE_OUTER_RADIUS,
                ur_round_radius: 2.0,
                lr_round_radius: supercube::CAVITY_ROUNDING,
            }
        )
    );
    // Bottom-left port
    println!(
        "{}",
        utils::gen_rounded_corner_rect(
            &mut dxf_writer,
            &utils::RoundedCornerRect {
                rect: geom::Rect{
                    ll: geom::Vec2 {
                        x: -supercube::CAVITY_WIDTH/2.0,
                        y: -supercube::CAVITY_HEIGHT/2.0,
                    },
                    ur: geom::Vec2 {
                        x: -supercube::WOOFER_MOUNT_BRIDGE_THICKNESS/2.0,
                        y: -supercube::WOOFER_MOUNT_BRIDGE_THICKNESS/2.0,
                    },
                },
                ll_round_radius: supercube::CAVITY_ROUNDING,
                ul_round_radius: 2.0,
                ur_round_radius: -supercube::TUBE_OUTER_RADIUS,
                lr_round_radius: 2.0,
            }
        )
    );
    // Top-left port
    println!(
        "{}",
        utils::gen_rounded_corner_rect(
            &mut dxf_writer,
            &utils::RoundedCornerRect {
                rect: geom::Rect{
                    ll: geom::Vec2 {
                        x: -supercube::CAVITY_WIDTH/2.0,
                        y: supercube::WOOFER_MOUNT_BRIDGE_THICKNESS/2.0,
                    },
                    ur: geom::Vec2 {
                        x: -supercube::WOOFER_MOUNT_BRIDGE_THICKNESS/2.0,
                        y: supercube::CAVITY_HEIGHT/2.0,
                    },
                },
                ll_round_radius: 2.0,
                ul_round_radius: supercube::CAVITY_ROUNDING,
                ur_round_radius: 2.0,
                lr_round_radius: -supercube::TUBE_OUTER_RADIUS,
            }
        )
    );

    // pilot holes
    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                  center: utils::origin(),
                  radius: supercube::WOOFER_BOLT_CIRCLE_RADIUS,
                },
                num_holes: 4,
                hole_radius: supercube::PILOT_HOLE_RADIUS,
                angle_offset: 45.0,
            }
        )
    );

    /*
    // Cavity
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
    */

    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);
    Ok(())
}
