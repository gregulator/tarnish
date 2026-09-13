// PARAMETERS

// Dimensions are in mm unless otherwise specified.
// Angles are in degrees.

// const PI: f64 = std::f64::consts::PI;
const SQRT_2: f64 = std::f64::consts::SQRT_2;

// Outer dimensions of the speaker.
pub const OUTER_WIDTH: f64 = 200.0;
pub const OUTER_HEIGHT: f64 = 200.0;

// Thickness of outer wall.
pub const OUTER_WALL_THICKNESS: f64 = 12.5;

// Rounding of the outer corners.
pub const OUTER_ROUNDING: f64 = 50.0;

// Computed dimensions of cavity
pub const CAVITY_WIDTH: f64 = OUTER_WIDTH - 2.0*OUTER_WALL_THICKNESS;
pub const CAVITY_HEIGHT: f64 = OUTER_HEIGHT - 2.0*OUTER_WALL_THICKNESS;

// Rounding of the cavity corners.
// Intentionally beefy at the corners.
pub const CAVITY_ROUNDING: f64 = 50.0;

// Thickness of front panel wall.
// This is slightly thicker than the wood walls.
pub const FRONT_PANEL_WALL_THICKNESS: f64 = 17.5;
// Computed dimensions of front panel cavity
pub const FRONT_PANEL_CAVITY_WIDTH: f64 = OUTER_WIDTH - 2.0*FRONT_PANEL_WALL_THICKNESS;
pub const FRONT_PANEL_CAVITY_HEIGHT: f64 = OUTER_HEIGHT - 2.0*FRONT_PANEL_WALL_THICKNESS;

// Front panel bolt circle radius
pub const FRONT_PANEL_BOLT_CIRCLE_RADIUS: f64 = OUTER_WIDTH*SQRT_2/2.0 - OUTER_ROUNDING/2.0 - OUTER_WALL_THICKNESS/2.0;

