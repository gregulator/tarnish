// Run with:
// cargo run --bin=baffle_main | tee baffle.dxf

use tarnish::air;
use tarnish::dxf;
use tarnish::geom;
use tarnish::utils;

const PI: f64 = std::f64::consts::PI;

// use std::fs::File;
// use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let side_angle = PI / 2.0 - air::BAFFLE_SIDE_ANGLE;

    let spine0 = geom::Vec2 { x: 0.0, y: 0.0 };
    let spine1 = geom::Vec2 {
        x: air::STAND_WING_LENGTH * f64::cos(side_angle),
        y: air::STAND_WING_LENGTH * f64::sin(side_angle),
    };
    let spine2 = geom::Vec2 {
        x: spine1.x + air::stand_center_spine_length(),
        y: spine1.y,
    };
    let spine3 = geom::Vec2 {
        x: spine2.x + air::STAND_WING_LENGTH * f64::cos(side_angle),
        y: 0.0,
    };

    let s0_a = utils::add(
        &spine0,
        &utils::scale(
            0.5 * air::STAND_BAR_THICKNESS,
            &utils::perp2(&spine1, &spine0),
        ),
    );
    let s1_a = utils::add(
        &spine1,
        &utils::scale(
            0.5 * air::STAND_BAR_THICKNESS,
            &utils::perp2(&spine1, &spine0),
        ),
    );
    let s2_a = utils::add(
        &spine2,
        &utils::scale(
            0.5 * air::STAND_BAR_THICKNESS,
            &utils::perp2(&spine3, &spine2),
        ),
    );
    let s3_a = utils::add(
        &spine3,
        &utils::scale(
            0.5 * air::STAND_BAR_THICKNESS,
            &utils::perp2(&spine3, &spine2),
        ),
    );

    let s0_b = utils::add(
        &spine0,
        &utils::scale(
            -0.5 * air::STAND_BAR_THICKNESS,
            &utils::perp2(&spine1, &spine0),
        ),
    );
    let s1_b = utils::add(
        &spine1,
        &utils::scale(
            -0.5 * air::STAND_BAR_THICKNESS,
            &utils::perp2(&spine1, &spine0),
        ),
    );
    let s2_b = utils::add(
        &spine2,
        &utils::scale(
            -0.5 * air::STAND_BAR_THICKNESS,
            &utils::perp2(&spine3, &spine2),
        ),
    );
    let s3_b = utils::add(
        &spine3,
        &utils::scale(
            -0.5 * air::STAND_BAR_THICKNESS,
            &utils::perp2(&spine3, &spine2),
        ),
    );

    let total_middle_x = (spine1.x + spine2.x) / 2.0;

    let mut dxf_writer = dxf::DxfWriter::new();
    let extent = geom::Bounds2 {
        min: geom::Vec2 { x: 0.0, y: 0.0 },
        max: geom::Vec2 {
            x: 1000.0,
            y: 1000.0,
        },
    };

    // Figure out the offset of the baffle stand holes from the ground & center line.
    let baffle_lower_stand_hole_y_offset_to_bend_line =
        air::BAFFLE_LOWER_STAND_HOLE_CENTER_Y - air::BAFFLE_PART_THICKNESS / 2.0;
    let baffle_upper_stand_hole_y_offset_to_bend_line =
        air::BAFFLE_UPPER_STAND_HOLE_CENTER_Y - air::BAFFLE_PART_THICKNESS / 2.0;

    let bend0_a = utils::add(
        &s0_a,
        &utils::scale(
            air::STAND_WING_LENGTH,
            &utils::normalize(&utils::sub(&spine1, &spine0)),
        ),
    );
    let bend0_b = utils::add(
        &s0_b,
        &utils::scale(
            air::STAND_WING_LENGTH,
            &utils::normalize(&utils::sub(&spine1, &spine0)),
        ),
    );
    let bend0_center = geom::Vec2 {
        x: (bend0_a.x + bend0_b.x) / 2.0,
        y: (bend0_a.y + bend0_b.y) / 2.0,
    };
    let bend1_a = utils::add(
        &s3_a,
        &utils::scale(
            -air::STAND_WING_LENGTH,
            &utils::normalize(&utils::sub(&spine3, &spine2)),
        ),
    );
    let bend1_b = utils::add(
        &s3_b,
        &utils::scale(
            -air::STAND_WING_LENGTH,
            &utils::normalize(&utils::sub(&spine3, &spine2)),
        ),
    );
    let bend1_center = geom::Vec2 {
        x: (bend1_a.x + bend1_b.x) / 2.0,
        y: (bend1_a.y + bend1_b.y) / 2.0,
    };
    // Project offsets to bend line
    let left_bar_origin = &bend0_center;
    let left_bar_y_axis = &utils::normalize(&utils::sub(&spine0, &spine1));
    let left_bar_x_axis = &utils::perp(left_bar_y_axis);

    let right_bar_origin = &bend1_center;
    let right_bar_y_axis = &utils::normalize(&utils::sub(&spine3, &spine2));
    let right_bar_x_axis = &utils::perp(right_bar_y_axis);

    let stand_ll_hole_center = utils::transform_point(
        &left_bar_origin,
        &left_bar_x_axis,
        &left_bar_y_axis,
        &geom::Vec2 {
            x: air::BAFFLE_STAND_HOLE_OFFSET_X,
            y: baffle_lower_stand_hole_y_offset_to_bend_line,
        },
    );
    let stand_ul_hole_center = utils::transform_point(
        &left_bar_origin,
        &left_bar_x_axis,
        &left_bar_y_axis,
        &geom::Vec2 {
            x: -air::BAFFLE_STAND_HOLE_OFFSET_X,
            y: baffle_upper_stand_hole_y_offset_to_bend_line,
        },
    );
    let stand_lr_hole_center = utils::transform_point(
        &right_bar_origin,
        &right_bar_x_axis,
        &right_bar_y_axis,
        &geom::Vec2 {
            x: air::BAFFLE_STAND_HOLE_OFFSET_X,
            y: baffle_lower_stand_hole_y_offset_to_bend_line,
        },
    );
    let stand_ur_hole_center = utils::transform_point(
        &right_bar_origin,
        &right_bar_x_axis,
        &right_bar_y_axis,
        &geom::Vec2 {
            x: -air::BAFFLE_STAND_HOLE_OFFSET_X,
            y: baffle_upper_stand_hole_y_offset_to_bend_line,
        },
    );

    // BAFFLE_LL_STAND_HOLE_CENTER_X is
    // Bolt hole offsets from the bend center n
    let b0 = geom::Vec2 {
        x: s1_a.x + (s2_a.x - s1_a.x) / 2.0 - air::STAND_BACK_WIDTH / 2.0,
        y: air::STAND_DEPTH,
    };
    let b1 = geom::Vec2 {
        x: s2_a.x - ((s2_a.x - s1_a.x) / 2.0 - air::STAND_BACK_WIDTH / 2.0),
        y: air::STAND_DEPTH,
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
                    point: utils::copy(&s0_a),
                    bulge: None
                },
                geom::PolylineVertex {
                    point: utils::copy(&s1_a),
                    bulge: Some(0.35)
                },
                geom::PolylineVertex {
                    point: b0,
                    bulge: Some(-0.25)
                },
                geom::PolylineVertex {
                    point: b1,
                    bulge: Some(0.35)
                },
                geom::PolylineVertex {
                    point: utils::copy(&s2_a),
                    bulge: None
                },
                geom::PolylineVertex {
                    point: utils::copy(&s3_a),
                    bulge: Some(-1.0)
                },
                geom::PolylineVertex {
                    point: utils::copy(&s3_b),
                    bulge: None
                },
                geom::PolylineVertex {
                    point: s2_b,
                    bulge: Some(0.15)
                },
                geom::PolylineVertex {
                    point: s1_b,
                    bulge: None
                },
                geom::PolylineVertex {
                    point: utils::copy(&s0_b),
                    bulge: Some(-1.0)
                },
                geom::PolylineVertex {
                    point: utils::copy(&s0_a),
                    bulge: None
                },
            ],
        })
    );

    // CUTOUTS

    // Stand attachement bolt holes.
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: stand_ll_hole_center,
            radius: 4.0
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: stand_ul_hole_center,
            radius: 4.0
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: stand_ur_hole_center,
            radius: 4.0
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: stand_lr_hole_center,
            radius: 4.0
        })
    );

    // stud cutouts for crossover board
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: total_middle_x - air::STAND_XOVER_HOLE_SPACING_X / 2.0,
                y: air::STAND_XOVER_CENTER_Y - air::STAND_XOVER_HOLE_SPACING_Y / 2.0,
            },
            radius: air::STAND_XOVER_HOLE_RADIUS
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: total_middle_x + air::STAND_XOVER_HOLE_SPACING_X / 2.0,
                y: air::STAND_XOVER_CENTER_Y - air::STAND_XOVER_HOLE_SPACING_Y / 2.0,
            },
            radius: air::STAND_XOVER_HOLE_RADIUS
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: total_middle_x + air::STAND_XOVER_HOLE_SPACING_X / 2.0,
                y: air::STAND_XOVER_CENTER_Y + air::STAND_XOVER_HOLE_SPACING_Y / 2.0,
            },
            radius: air::STAND_XOVER_HOLE_RADIUS
        })
    );
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: total_middle_x - air::STAND_XOVER_HOLE_SPACING_X / 2.0,
                y: air::STAND_XOVER_CENTER_Y + air::STAND_XOVER_HOLE_SPACING_Y / 2.0,
            },
            radius: air::STAND_XOVER_HOLE_RADIUS
        })
    );

    // terminal cutouts
    println!(
        "{}",
        utils::gen_terminal_hole_with_notch(
            &mut dxf_writer,
            &utils::TerminalHoleWithNotch {
                hole_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: (s2_a.x + s1_a.x) / 2.0 - air::STAND_TERMINAL_SPACING / 2.0,
                        y: air::STAND_TERMINAL_Y,
                    },
                    radius: air::STAND_TERMINAL_RADIUS
                },
                notch_length: air::STAND_TERMINAL_NOTCH,
            }
        )
    );
    println!(
        "{}",
        utils::gen_terminal_hole_with_notch(
            &mut dxf_writer,
            &utils::TerminalHoleWithNotch {
                hole_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: (s2_a.x + s1_a.x) / 2.0 + air::STAND_TERMINAL_SPACING / 2.0,
                        y: air::STAND_TERMINAL_Y,
                    },
                    radius: air::STAND_TERMINAL_RADIUS
                },
                notch_length: air::STAND_TERMINAL_NOTCH,
            }
        )
    );

    // BENDLINES
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: bend0_a,
            p1: bend0_b,
        })
    );
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: bend1_a,
            p1: bend1_b,
        })
    );
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: geom::Vec2 { x: 235.0, y: 350.0 },
            p1: geom::Vec2 { x: 610.0, y: 350.0 }
        })
    );
    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);

    Ok(())
}
