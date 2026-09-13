// Run with:
// cargo run --bin=supercube_c_tweetermount_main | tee supercube_c_tweetermount.dxf

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
    // Tweeter cutout
    println!(
        "{}",
        &dxf_writer.gen_circle(
            geom::Circle {
               center: utils::origin(),
               radius: supercube::TWEETER_CUTOUT_RADIUS,
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
                        x: supercube::TWEETER_MOUNT_BRIDGE_THICKNESS/2.0,
                        y: supercube::TWEETER_MOUNT_BRIDGE_THICKNESS/2.0,
                    },
                    ur: geom::Vec2 {
                        x: (supercube::CAVITY_WIDTH - supercube::TWEETER_MOUNT_BRIDGE_THICKNESS)/2.0,
                        y: (supercube::CAVITY_HEIGHT - supercube::TWEETER_MOUNT_BRIDGE_THICKNESS)/2.0,
                    },
                },
                ll_round_radius: -supercube::TWEETER_MOUNT_RADIUS,
                ul_round_radius: 5.0,
                ur_round_radius: supercube::CAVITY_ROUNDING,
                lr_round_radius: 5.0,
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
                        x: supercube::TWEETER_MOUNT_BRIDGE_THICKNESS/2.0,
                        y: -(supercube::CAVITY_HEIGHT - supercube::TWEETER_MOUNT_BRIDGE_THICKNESS)/2.0,
                    },
                    ur: geom::Vec2 {
                        x: (supercube::CAVITY_WIDTH - supercube::TWEETER_MOUNT_BRIDGE_THICKNESS)/2.0,
                        y: -supercube::TWEETER_MOUNT_BRIDGE_THICKNESS/2.0,
                    },
                },
                ll_round_radius: 5.0,
                ul_round_radius: -supercube::TWEETER_MOUNT_RADIUS,
                ur_round_radius: 5.0,
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
                        x: -(supercube::CAVITY_WIDTH - supercube::TWEETER_MOUNT_BRIDGE_THICKNESS)/2.0,
                        y: -(supercube::CAVITY_HEIGHT - supercube::TWEETER_MOUNT_BRIDGE_THICKNESS)/2.0,
                    },
                    ur: geom::Vec2 {
                        x: -supercube::TWEETER_MOUNT_BRIDGE_THICKNESS/2.0,
                        y: -supercube::TWEETER_MOUNT_BRIDGE_THICKNESS/2.0,
                    },
                },
                ll_round_radius: supercube::CAVITY_ROUNDING,
                ul_round_radius: 5.0,
                ur_round_radius: -supercube::TWEETER_MOUNT_RADIUS,
                lr_round_radius: 5.0,
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
                        x: -(supercube::CAVITY_WIDTH - supercube::TWEETER_MOUNT_BRIDGE_THICKNESS)/2.0,
                        y: supercube::TWEETER_MOUNT_BRIDGE_THICKNESS/2.0,
                    },
                    ur: geom::Vec2 {
                        x: -supercube::TWEETER_MOUNT_BRIDGE_THICKNESS/2.0,
                        y: (supercube::CAVITY_HEIGHT - supercube::TWEETER_MOUNT_BRIDGE_THICKNESS)/2.0,
                    },
                },
                ll_round_radius: 5.0,
                ul_round_radius: supercube::CAVITY_ROUNDING,
                ur_round_radius: 5.0,
                lr_round_radius: -supercube::TWEETER_MOUNT_RADIUS,
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
    */

    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);
    Ok(())
}
