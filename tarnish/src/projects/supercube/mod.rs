// PARAMETERS

// Dimensions are in mm unless otherwise specified.
// Angles are in degrees.

// const PI: f64 = std::f64::consts::PI;
const SQRT_2: f64 = std::f64::consts::SQRT_2;

// Outer dimensions of the speaker, not including veneer.
pub const OUTER_WIDTH: f64 = 220.0;
pub const OUTER_HEIGHT: f64 = 220.0;

// Anticipated width of veneer. The metal front & back panels are oversized by
// this amount in each direction.
pub const VENEER_WIDTH: f64 = 1.0;

// Thickness of outer wall.
pub const OUTER_WALL_THICKNESS: f64 = 12.0;

// Rounding of the outer corners.
pub const OUTER_ROUNDING: f64 = 60.0;

// Computed dimensions of cavity
pub const CAVITY_WIDTH: f64 = OUTER_WIDTH - 2.0*OUTER_WALL_THICKNESS;
pub const CAVITY_HEIGHT: f64 = OUTER_HEIGHT - 2.0*OUTER_WALL_THICKNESS;

// Rounding of the cavity corners.
// Intentionally beefy at the corners.
pub const CAVITY_ROUNDING: f64 = 50.0;

// Thickness of front panel wall.
// This is slightly thicker than the wood walls.
pub const FRONT_PANEL_WALL_THICKNESS: f64 = 13.0;
// Computed dimensions of front panel cavity
pub const FRONT_PANEL_CAVITY_WIDTH: f64 = OUTER_WIDTH - 2.0*FRONT_PANEL_WALL_THICKNESS;
pub const FRONT_PANEL_CAVITY_HEIGHT: f64 = OUTER_HEIGHT - 2.0*FRONT_PANEL_WALL_THICKNESS;

// Front panel bolt circle radius
pub const FRONT_PANEL_BOLT_CIRCLE_RADIUS: f64 = OUTER_WIDTH*SQRT_2/2.0 - OUTER_ROUNDING/2.35 - OUTER_WALL_THICKNESS/2.0;

// Terminals
pub const TERMINAL_NOTCH_LENGTH: f64 = 2.5;
pub const TERMINAL_DIST_FROM_CAVITY_BOTTOM: f64 = 18.0;
pub const TERMINAL_SPACING: f64 = 50.0;
pub const TERMINAL_RADIUS: f64 = 12.5/2.0; // Post is 11.5
pub const TERMINAL_Y: f64 = -CAVITY_HEIGHT/2.0 + TERMINAL_DIST_FROM_CAVITY_BOTTOM + TERMINAL_RADIUS;

// Tweeter
pub const TWEETER_CUTOUT_RADIUS: f64 = 37.5; // 29HD2 71mm; Scanspeak D2904/980000 73.5mm cutout -> 75/2 -> 37.5
pub const TWEETER_MOUNT_RADIUS: f64 = 49.0; // Oversized; manually verified; imperfect.
// Width of the four bridges that connect the tweeter mounting circle to the outer wall.
pub const TWEETER_MOUNT_BRIDGE_THICKNESS: f64 = 16.0;
pub const TWEETER_BOLT_CIRCLE_RADIUS: f64 = 42.5; // 29HD2 85mm diam -> 42.5
pub const TWEETER_TERMINAL_CUTOUT_WIDTH: f64 = 11.0;
pub const TWEETER_TERMINAL_CUTOUT_HEIGHT: f64 = 5.0;
pub const TWEETER_TERMINAL_CUTOUT_OFFSET_Y: f64 = 0.0;

// Woofer
pub const WOOFER_CUTOUT_RADIUS: f64 = 65.0; //TODO: Check this.  Scanspeak D2904/980000 - 126mm cutout -> 134/2 -> 67
//pub const WOOFER_MOUNT_RADIUS: f64 = 68.0; // TODO: Check this ??? Scanspeak D2904/980000 - 104.3mm outer radius -> 120/2 -> 60
pub const WOOFER_MOUNT_RADIUS: f64 = 78.0; // TODO: Check this ??? Scanspeak D2904/980000 - 104.3mm outer radius -> 120/2 -> 60
// Width of the four bridges that connect the tweeter mounting circle to the outer wall.
pub const WOOFER_MOUNT_BRIDGE_THICKNESS: f64 = 14.0;

pub const WOOFER_BOLT_CIRCLE_RADIUS: f64 = 70.0; // E150HE-44 140mm diam -> 70

// Outer radius of the woofer tube.
// Because of the bridge offsets, this isn't exact and should be measured for correctness in CAD software.
pub const TUBE_OUTER_RADIUS: f64 = 86.0; // TODO: Check this ??? Scanspeak D2904/980000 - 104.3mm outer radius -> 120/2 -> 60

pub const TUBE_INNER_RADIUS: f64 = 80.0; //TODO: Check this.  Scanspeak D2904/980000 - 126mm cutout -> 134/2 -> 67
// Crossover board
pub const XOVER_BOARD_WIDTH: f64 = 150.0;
pub const XOVER_BOARD_HEIGHT: f64 = 100.0;
pub const XOVER_BOARD_POST_DIST_TO_SIDE: f64 = 12.5;
pub const XOVER_BOARD_POST_WIDTH: f64 = 125.0-XOVER_BOARD_POST_DIST_TO_SIDE*2.0;
pub const XOVER_BOARD_POST_HEIGHT: f64 = 80.0-XOVER_BOARD_POST_DIST_TO_SIDE*2.0;


pub const PILOT_HOLE_RADIUS: f64 = 1.7;
