// Run with:
// cargo run --bin=supercube_b_cavitiy_main | tee supercube_b_cavity.dxf

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

    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);
    Ok(())
}
