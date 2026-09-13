// PARAMETERS

// Dimensions are in mm unless otherwise specified.

const PI: f64 = std::f64::consts::PI;

// Outer dimensions of the speaker.
pub const OUTER_WIDTH: f64 = 200.0;
pub const OUTER_HEIGHT: f64 = 200.0;

// Thickness of outer wall.
pub const OUTER_WALL_THICKNESS: f64 = 12.5;

// Rounding of the outer corners.
pub const OUTER_ROUNDING: f64 = 25.0;


// Computed dimensions of cavity
pub const CAVITY_WIDTH: f64 = OUTER_WIDTH - 2.0*OUTER_WALL_THICKNESS;
pub const CAVITY_HEIGHT: f64 = OUTER_HEIGHT - 2.0*OUTER_WALL_THICKNESS;

// Rounding of the cavity corners.
// Intentionally beefy at the corners.
pub const CAVITY_ROUNDING: f64 = 30.0;


