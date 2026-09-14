// PARAMETERS

// Dimensions are in mm unless otherwise specified.
// Angles are in degrees.

// const PI: f64 = std::f64::consts::PI;
const SQRT_2: f64 = std::f64::consts::SQRT_2;

// Outer dimensions of the speaker.
pub const OUTER_WIDTH: f64 = 215.0;
pub const OUTER_HEIGHT: f64 = 215.0;

// Thickness of outer wall.
pub const OUTER_WALL_THICKNESS: f64 = 12.0;

// Rounding of the outer corners.
pub const OUTER_ROUNDING: f64 = 48.0;

// Computed dimensions of cavity
pub const CAVITY_WIDTH: f64 = OUTER_WIDTH - 2.0*OUTER_WALL_THICKNESS;
pub const CAVITY_HEIGHT: f64 = OUTER_HEIGHT - 2.0*OUTER_WALL_THICKNESS;

// Rounding of the cavity corners.
// Intentionally beefy at the corners.
pub const CAVITY_ROUNDING: f64 = 40.0;

// Thickness of front panel wall.
// This is slightly thicker than the wood walls.
pub const FRONT_PANEL_WALL_THICKNESS: f64 = 17.5;
// Computed dimensions of front panel cavity
pub const FRONT_PANEL_CAVITY_WIDTH: f64 = OUTER_WIDTH - 2.0*FRONT_PANEL_WALL_THICKNESS;
pub const FRONT_PANEL_CAVITY_HEIGHT: f64 = OUTER_HEIGHT - 2.0*FRONT_PANEL_WALL_THICKNESS;

// Front panel bolt circle radius
pub const FRONT_PANEL_BOLT_CIRCLE_RADIUS: f64 = OUTER_WIDTH*SQRT_2/2.0 - OUTER_ROUNDING/2.0 - OUTER_WALL_THICKNESS/2.0;

// Terminals
pub const TERMINAL_NOTCH_LENGTH: f64 = 2.5;
pub const TERMINAL_DIST_FROM_CAVITY_BOTTOM: f64 = 18.0;
pub const TERMINAL_SPACING: f64 = 50.0;
pub const TERMINAL_RADIUS: f64 = 12.5/2.0; // Post is 11.5
pub const TERMINAL_Y: f64 = -CAVITY_HEIGHT/2.0 + TERMINAL_DIST_FROM_CAVITY_BOTTOM + TERMINAL_RADIUS;

// Tweeter
pub const TWEETER_CUTOUT_RADIUS: f64 = 40.0; // Scanspeak D2904/980000 - 73.5mm cutout -> 80/2 -> 40
pub const TWEETER_MOUNT_RADIUS: f64 = 50.0; // TODO: Check this ??? Scanspeak D2904/980000 - 104.3mm outer radius -> 120/2 -> 60
// Width of the four bridges that connect the tweeter mounting circle to the outer wall.
pub const TWEETER_MOUNT_BRIDGE_THICKNESS: f64 = 12.5;

// Woofer
pub const WOOFER_CUTOUT_RADIUS: f64 = 65.0; //TODO: Check this.  Scanspeak D2904/980000 - 126mm cutout -> 134/2 -> 67
//pub const WOOFER_MOUNT_RADIUS: f64 = 68.0; // TODO: Check this ??? Scanspeak D2904/980000 - 104.3mm outer radius -> 120/2 -> 60
pub const WOOFER_MOUNT_RADIUS: f64 = 78.0; // TODO: Check this ??? Scanspeak D2904/980000 - 104.3mm outer radius -> 120/2 -> 60
// Width of the four bridges that connect the tweeter mounting circle to the outer wall.
pub const WOOFER_MOUNT_BRIDGE_THICKNESS: f64 = 14.0;

// Outer radius of the woofer tube.
// Because of the bridge offsets, this isn't exact and should be measured for correctness in CAD software.
pub const TUBE_OUTER_RADIUS: f64 = 84.0; // TODO: Check this ??? Scanspeak D2904/980000 - 104.3mm outer radius -> 120/2 -> 60

pub const TUBE_INNER_RADIUS: f64 = 80.0; //TODO: Check this.  Scanspeak D2904/980000 - 126mm cutout -> 134/2 -> 67
// Crossover board
pub const XOVER_BOARD_WIDTH: f64 = 125.0;
pub const XOVER_BOARD_HEIGHT: f64 = 80.0;
pub const XOVER_BOARD_POST_DIST_TO_SIDE: f64 = 12.5;
