// Run with:
// cargo run --bin=baffle_main | tee baffle.dxf

// TODO:
// - Set units to MM (Done but SCS still checks)
// - Rounded rectangles
// - Bolt hole rings
// - Gen handles (not really necessary)
// - Polygons
// - Splines
use tarnish::air;
use tarnish::dxf;
use tarnish::geom;
use tarnish::utils;

// use std::fs::File;
// use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // PARAMETERS
    let show_trim_outlines = false;

    // Width of leg sections.
    let baffle_leg_length = 25.0;

    // Amount to round corners.
    let baffle_corner_rounding = 3.0;

    // Amount to round corners.
    let baffle_corner_rounding_buldge = -0.4;

    // Gap between the floor and the bottom of the

    // DERIVED PARAMETERS
    let baffle_wing_width = (air::BAFFLE_WIDTH - air::BAFFLE_CENTER_WIDTH) / 2.0;

    let mut dxf_writer = dxf::DxfWriter::new();

    let extent = geom::Bounds2 {
        min: geom::Vec2 { x: 0.0, y: 0.0 },
        max: geom::Vec2 {
            x: air::BAFFLE_WIDTH,
            y: air::BAFFLE_HEIGHT,
        },
    };
    println!("{}", dxf::gen_header(extent));
    println!("{}", dxf::TABLES);
    println!("{}", dxf::BLOCKS);
    println!("{}", dxf::ENTITIES_HEADER);
    // CHECK ALL OF THIS BEFORE ORDERING!

    // OUTLINE SHAPE
    // Note: bulge applies to the _next_ line.
    println!(
        "{}",
        dxf_writer.gen_polyline(geom::Polyline {
            v: vec![
                // * bottom-left
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: baffle_corner_rounding,
                        y: 0.0
                    },
                    bulge: Some(baffle_corner_rounding_buldge)
                },
                // <- Rounded bottom-left corner
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 0.0,
                        y: baffle_corner_rounding
                    },
                    bulge: None
                },
                // <- left-side from floor to taper start ->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: 0.0,
                        y: air::BAFFLE_TAPER_START_Y
                    },
                    bulge: Some(0.05)
                },
                // <- left-side taper to top of baffle ->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: baffle_wing_width - air::BAFFLE_TOP_OVERHANG,
                        y: air::BAFFLE_HEIGHT - baffle_corner_rounding,
                    },
                    bulge: Some(0.0 * baffle_corner_rounding_buldge)
                },
                // <- top-left rounded corner ->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: baffle_wing_width - air::BAFFLE_TOP_OVERHANG + baffle_corner_rounding,
                        y: air::BAFFLE_HEIGHT,
                    },
                    bulge: Some(-0.1)
                },
                // <- top ->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::BAFFLE_WIDTH - baffle_wing_width + air::BAFFLE_TOP_OVERHANG
                            - baffle_corner_rounding,
                        y: air::BAFFLE_HEIGHT,
                    },
                    bulge: Some(0.0 * baffle_corner_rounding_buldge)
                },
                // <- top-right rounded corner ->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::BAFFLE_WIDTH - baffle_wing_width + air::BAFFLE_TOP_OVERHANG,
                        y: air::BAFFLE_HEIGHT - baffle_corner_rounding,
                    },
                    bulge: Some(0.05)
                },
                // <- right-side taper from top of baffle ->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::BAFFLE_WIDTH,
                        y: air::BAFFLE_TAPER_START_Y
                    },
                    bulge: None
                },
                // <- right side from taper start to floor ->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::BAFFLE_WIDTH,
                        y: baffle_corner_rounding
                    },
                    bulge: Some(baffle_corner_rounding_buldge)
                },
                // <- rounded bottom-right corner
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::BAFFLE_WIDTH - baffle_corner_rounding,
                        y: 0.0
                    },
                    bulge: None
                },
                // <- right-rear leg -->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::BAFFLE_WIDTH - baffle_leg_length,
                        y: 0.0
                    },
                    bulge: Some(0.5)
                },
                // <- right side bottom gap -->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::BAFFLE_WIDTH - baffle_wing_width + baffle_leg_length,
                        y: 0.0
                    },
                    bulge: None
                },
                // <- front-right leg -->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: air::BAFFLE_WIDTH - baffle_wing_width - baffle_leg_length,
                        y: 0.0
                    },
                    bulge: Some(0.25)
                },
                // <- front bottom gap -->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: baffle_wing_width + baffle_leg_length,
                        y: 0.0
                    },
                    bulge: None
                },
                // <- front-bottom gap -->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: baffle_wing_width - baffle_leg_length,
                        y: 0.0
                    },
                    bulge: Some(0.5)
                },
                // <- left-rear leg ->
                geom::PolylineVertex {
                    point: geom::Vec2 {
                        x: baffle_leg_length,
                        y: 0.0
                    },
                    bulge: None
                },
            ],
        })
    );

    // LOWER WOOFER
    // (DEBUG) Trim.
    if show_trim_outlines {
        println!(
            "{}",
            dxf_writer.gen_circle(geom::Circle {
                center: geom::Vec2 {
                    x: air::BAFFLE_CENTER_X,
                    y: air::BAFFLE_LOWER_WOOF_CENTER_Y,
                },
                radius: air::WOOFTRIM_OUTER_RADIUS,
            })
        );
    }
    // Cutout
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: air::BAFFLE_CENTER_X,
                y: air::BAFFLE_LOWER_WOOF_CENTER_Y,
            },
            radius: air::WOOFER_CUTOUT_RADIUS,
        })
    );
    // BoltCircle
    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: air::BAFFLE_CENTER_X,
                        y: air::BAFFLE_LOWER_WOOF_CENTER_Y
                    },
                    radius: air::WOOFER_BOLT_CIRCLE_RADIUS,
                },
                num_holes: 8,
                hole_radius: air::WOOFGASKET_BOLT_HOLE_RADIUS,
                angle_offset: 22.5,
            }
        )
    );

    // UPPER WOOFER
    // (DEBUG) Trim.
    if show_trim_outlines {
        println!(
            "{}",
            dxf_writer.gen_circle(geom::Circle {
                center: geom::Vec2 {
                    x: air::BAFFLE_CENTER_X,
                    y: air::BAFFLE_UPPER_WOOF_CENTER_Y,
                },
                radius: air::WOOFTRIM_OUTER_RADIUS,
            })
        );
    }
    // Cutout
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: air::BAFFLE_CENTER_X,
                y: air::BAFFLE_UPPER_WOOF_CENTER_Y,
            },
            radius: air::WOOFER_CUTOUT_RADIUS,
        })
    );
    // BoltCircle
    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: air::BAFFLE_CENTER_X,
                        y: air::BAFFLE_UPPER_WOOF_CENTER_Y
                    },
                    radius: air::WOOFER_BOLT_CIRCLE_RADIUS,
                },
                num_holes: 8,
                hole_radius: air::WOOFGASKET_BOLT_HOLE_RADIUS,
                angle_offset: 22.5,
            }
        )
    );

    // FULLRANGE TRIM
    if show_trim_outlines {
        println!(
            "{}",
            dxf_writer.gen_rect(geom::Rect {
                ll: geom::Vec2 {
                    x: air::FRTRIM_LL_X,
                    y: air::FRTRIM_LL_Y
                },
                ur: geom::Vec2 {
                    x: air::FRTRIM_UR_X,
                    y: air::FRTRIM_UR_Y
                }
            })
        );
    }

    // LOWER FULLRANGE
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: air::BAFFLE_CENTER_X,
                y: air::BAFFLE_LOWER_FR_CENTER_Y
            },
            radius: air::FR_CUTOUT_DIAMETER / 2.0,
        })
    );

    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: air::BAFFLE_CENTER_X,
                        y: air::BAFFLE_LOWER_FR_CENTER_Y
                    },
                    radius: air::FR_BOLT_CIRCLE_DIAMETER / 2.0,
                },
                num_holes: 4,
                hole_radius: air::BAFFLE_FR_BOLT_HOLE_DIAMETER / 2.0,
                angle_offset: 45.0,
            }
        )
    );

    // UPPER FULLRANGE
    println!(
        "{}",
        dxf_writer.gen_circle(geom::Circle {
            center: geom::Vec2 {
                x: air::BAFFLE_CENTER_X,
                y: air::BAFFLE_UPPER_FR_CENTER_Y
            },
            radius: air::FR_CUTOUT_DIAMETER / 2.0,
        })
    );

    println!(
        "{}",
        utils::gen_bolt_circle(
            &mut dxf_writer,
            utils::BoltCircle {
                ring_circle: geom::Circle {
                    center: geom::Vec2 {
                        x: air::BAFFLE_CENTER_X,
                        y: air::BAFFLE_UPPER_FR_CENTER_Y
                    },
                    radius: air::FR_BOLT_CIRCLE_DIAMETER / 2.0,
                },
                num_holes: 4,
                hole_radius: air::BAFFLE_FR_BOLT_HOLE_DIAMETER / 2.0,
                angle_offset: 45.0,
            }
        )
    );

    // STAND MOUNTING HOLES
    // Lower-left
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
    // Upper-left
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
    // Lower-right
    println!(
        "{}",
        utils::gen_pill(
            &mut dxf_writer,
            &utils::Pill {
                start: geom::Vec2 {
                    x: air::BAFFLE_LR_STAND_HOLE_CENTER_X - air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_LOWER_STAND_HOLE_CENTER_Y
                },
                end: geom::Vec2 {
                    x: air::BAFFLE_LR_STAND_HOLE_CENTER_X + air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_LOWER_STAND_HOLE_CENTER_Y
                },
                thickness: air::BAFFLE_STAND_HOLE_THICKNESS,
            }
        )
    );
    // Upper-right
    println!(
        "{}",
        utils::gen_pill(
            &mut dxf_writer,
            &utils::Pill {
                start: geom::Vec2 {
                    x: air::BAFFLE_UR_STAND_HOLE_CENTER_X - air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_UPPER_STAND_HOLE_CENTER_Y
                },
                end: geom::Vec2 {
                    x: air::BAFFLE_UR_STAND_HOLE_CENTER_X + air::BAFFLE_STAND_HOLE_LENGTH / 2.0,
                    y: air::BAFFLE_UPPER_STAND_HOLE_CENTER_Y
                },
                thickness: air::BAFFLE_STAND_HOLE_THICKNESS,
            }
        )
    );

    // BENDLINES
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: geom::Vec2 {
                x: air::BAFFLE_BEND_0_X,
                y: 0.0
            },
            p1: geom::Vec2 {
                x: air::BAFFLE_BEND_0_X,
                y: air::BAFFLE_HEIGHT,
            },
        })
    );
    let bend_1_x = air::BAFFLE_WIDTH / 2.0 + air::BAFFLE_CENTER_WIDTH / 2.0;
    println!(
        "{}",
        dxf_writer.gen_bendline(geom::LineSeg {
            p0: geom::Vec2 {
                x: bend_1_x,
                y: 0.0
            },
            p1: geom::Vec2 {
                x: bend_1_x,
                y: air::BAFFLE_HEIGHT,
            },
        })
    );
    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);

    Ok(())
}
