// Run with:
// cargo run --bin=supercube_f_backpanel_main | tee supercube_f_backpanel.dxf

// TODO: Remove cavity
// TODO: Posts for mounting crossover

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

    // Left Terminal
    println!(
        "{}",
        utils::gen_terminal_hole_with_notch2(
            &mut dxf_writer,
            &utils::TerminalHoleWithNotch {
                hole_circle: geom::Circle {
                    center: geom::Vec2 {
                      x: -supercube::TERMINAL_SPACING/2.0,
                      y: supercube::TERMINAL_Y
                    },
                    radius: supercube::TERMINAL_RADIUS,
                },
                notch_length: supercube::TERMINAL_NOTCH_LENGTH
            }
        )
    );

    // Right Terminal
    println!(
        "{}",
        utils::gen_terminal_hole_with_notch2(
            &mut dxf_writer,
            &utils::TerminalHoleWithNotch {
                hole_circle: geom::Circle {
                    center: geom::Vec2 {
                      x: supercube::TERMINAL_SPACING/2.0,
                      y: supercube::TERMINAL_Y
                    },
                    radius: supercube::TERMINAL_RADIUS,
                },
                notch_length: supercube::TERMINAL_NOTCH_LENGTH
            }
        )
    );

    // Crossover posts - should have hardware inserted.
    println!(
        "{}",
        &dxf_writer.gen_circle(
            geom::Circle {
               center: geom::Vec2 {
                    x: supercube::XOVER_BOARD_POST_WIDTH/2.0,
                    y: supercube::XOVER_BOARD_POST_HEIGHT/2.0,
                },
                radius: 4.0
            }
        )
    );
    println!(
        "{}",
        &dxf_writer.gen_circle(
            geom::Circle {
               center: geom::Vec2 {
                    x: -supercube::XOVER_BOARD_POST_WIDTH/2.0,
                    y: supercube::XOVER_BOARD_POST_HEIGHT/2.0,
                },
                radius: 4.0
            }
        )
    );
    println!(
        "{}",
        &dxf_writer.gen_circle(
            geom::Circle {
               center: geom::Vec2 {
                    x: supercube::XOVER_BOARD_POST_WIDTH/2.0,
                    y: -supercube::XOVER_BOARD_POST_HEIGHT/2.0,
                },
                radius: 4.0
            }
        )
    );
    println!(
        "{}",
        &dxf_writer.gen_circle(
            geom::Circle {
               center: geom::Vec2 {
                    x: -supercube::XOVER_BOARD_POST_WIDTH/2.0,
                    y: -supercube::XOVER_BOARD_POST_HEIGHT/2.0,
                },
                radius: 4.0
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
