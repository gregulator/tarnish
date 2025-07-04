// Run with:
// cargo run --bin=wooftrim_main | tee wooftrim.dxf

use tarnish::air;
use tarnish::dxf;
use tarnish::geom;
use tarnish::utils;

// use std::fs::File;
// use std::io::prelude::*;

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
        utils::gen_rounded_rect(
            &mut dxf_writer,
            &utils::RoundedRect {
                ll: geom::Vec2 {
                    x: -air::FRTRIM_WIDTH / 2.0,
                    y: -air::FRTRIM_HEIGHT / 2.0
                },
                ur: geom::Vec2 {
                    x: air::FRTRIM_WIDTH / 2.0,
                    y: air::FRTRIM_HEIGHT / 2.0
                },
                round_radius: 20.0
            }
        )
    );

    // CUTOUTS

    // TOP FR
    let topfr_center_y = air::FR_FR_GAP / 2.0 + air::FR_OUTER_LENGTH / 2.0;
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: 0.0,
                y: topfr_center_y
            },
            radius: air::FRTRIM_CUTOUT_RADIUS,
        })
    );
    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: 0.0,
                        y: topfr_center_y
                    },
                    radius: air::FR_BOLT_CIRCLE_RADIUS,
                },
                num_holes: 4,
                hole_radius: air::FRTRIM_BOLT_HOLE_RADIUS,
                angle_offset: 45.0,
            }
        )
    );

    // BOTTOM FR
    let bottomfr_center_y = -air::FR_FR_GAP / 2.0 - air::FR_OUTER_LENGTH / 2.0;
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: 0.0,
                y: bottomfr_center_y
            },
            radius: air::FRTRIM_CUTOUT_RADIUS,
        })
    );
    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: 0.0,
                        y: bottomfr_center_y
                    },
                    radius: air::FR_BOLT_CIRCLE_RADIUS,
                },
                num_holes: 4,
                hole_radius: air::FRTRIM_BOLT_HOLE_RADIUS,
                angle_offset: 45.0,
            }
        )
    );
    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);
    Ok(())
}
