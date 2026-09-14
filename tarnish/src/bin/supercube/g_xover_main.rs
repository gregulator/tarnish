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
                    x: -supercube::XOVER_BOARD_WIDTH/2.0,
                    y: -supercube::XOVER_BOARD_HEIGHT/2.0
                },
                ur: geom::Vec2 {
                    x: supercube::XOVER_BOARD_WIDTH/2.0,
                    y: supercube::XOVER_BOARD_HEIGHT/2.0
                },
                round_radius: supercube::XOVER_BOARD_ROUNDING,
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
                radius: supercube::XOVER_POST_HOLE_RADIUS
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
                radius: supercube::XOVER_POST_HOLE_RADIUS
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
                radius: supercube::XOVER_POST_HOLE_RADIUS
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
                radius: supercube::XOVER_POST_HOLE_RADIUS
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
