// PARAMETERS

const PI: f64 = std::f64::consts::PI;

pub const BAFFLE_PART_THICKNESS: f64 = 3.175;

pub const BAFFLE_SIDE_ANGLE: f64 = 60.0 * PI / 180.0;

// Overall baffle width (pre-bend).
pub const BAFFLE_WIDTH: f64 = 760.0;

// Baffle center section width.
pub const BAFFLE_CENTER_WIDTH: f64 = 434.0;

// How much does the top overhang the sides before the taper starts.
// (necessary for bends to work)
pub const BAFFLE_TOP_OVERHANG: f64 = 25.0;

// Height from floor at which the side's taper start.
pub const BAFFLE_TAPER_START_Y: f64 = 400.0;

// Gap between the floor and the bottom of the lower sub's trim.
pub const BAFFLE_BOTTOM_GAP_Y: f64 = 68.0;

// Gap between sub trims.
pub const BAFFLE_WOOF_WOOF_GAP_Y: f64 = 18.0;

// Gap between top of upper sub trim and the bottom of fullrange's trim rectangle.
pub const BAFFLE_WOOF_FR_GAP_Y: f64 = 18.0;

// Gap between top the FR trim and the top of the baffle.
pub const BAFFLE_TOP_GAP_Y: f64 = -0.0;

pub const FRTRIM_ROUNDING: f64 = 10.0;

// Woofer cutout diameter (GRS 15-Pro)
pub const WOOFER_CUTOUT_DIAMETER: f64 = 360.0;

// Woofer bolt circle radius (GRS 15-Pro)
pub const WOOFER_BOLT_CIRCLE_DIAMETER: f64 = 372.0;

// FR cutout diameter (DMA70-8) (Spec-sheet says 64)
pub const FR_CUTOUT_DIAMETER: f64 = 66.0;

// FR bolt circle diameter (DMA70-8) (from spec sheet)
pub const FR_BOLT_CIRCLE_DIAMETER: f64 = 84.1;

// FR outer side length
pub const FR_OUTER_LENGTH: f64 = 70.0;

// spacing between FR drivers.
pub const FR_FR_GAP: f64 = 6.0;

// spacing between FR TRIM edge and FR Driver edge.
pub const FRTRIM_PADDING_Y: f64 = 12.5;

pub const FRTRIM_PADDING_X: f64 = 12.5;

// Woofer trim thickness.
pub const WOOFTRIM_THICKNESS: f64 = 25.0;

// Woofer gasket thickness.
pub const WOOFGASKET_THICKNESS: f64 = 16.0;

// Woofer trim bolt hole diameter.
// Needs to fit #8-32 barrel nut w/ 13/64" diameter.
// SO 5.159375mm + oversize a bit.
// For the metal trim it doesn't really matter since hardware gets inserted.
// But this is also used for the gasket, so should be oversized.
pub const WOOFTRIM_BOLT_HOLE_DIAMETER: f64 = 6.16;

// Needs to fit #4 hex standoff w/ 3/16" short diagonal
// See: https://www.gigacalculator.com/calculators/hexagon-calculator.php
// 5.5mm long diagonal + oversize a little bit.
pub const BAFFLE_FR_BOLT_HOLE_DIAMETER: f64 = 6.5;

pub const BAFFLE_LOWER_STAND_HOLE_CENTER_Y: f64 = 50.0;
pub const BAFFLE_UPPER_STAND_HOLE_CENTER_Y: f64 = 150.0;
pub const BAFFLE_STAND_HOLE_OFFSET_X: f64 = -4.0;
pub const BAFFLE_STAND_HOLE_LENGTH: f64 = 12.5;
pub const BAFFLE_STAND_HOLE_THICKNESS: f64 = 8.0;

pub const STAND_WING_LENGTH: f64 = 200.0;
pub const STAND_DEPTH: f64 = 450.0;
pub const STAND_BACK_WIDTH: f64 = 300.0;

pub const STAND_TERMINAL_SPACING: f64 = 50.0;
pub const STAND_TERMINAL_RADIUS: f64 = 4.8; // 3/16" -> 4.8mm
pub const STAND_TERMINAL_NOTCH: f64 = 1.0;
pub const STAND_TERMINAL_Y: f64 = 450.0;

pub const STAND_XOVER_HOLE_SPACING_X: f64 = 170.0;
pub const STAND_XOVER_HOLE_SPACING_Y: f64 = 117.5;

// Needs to sit on #4 hex standoff w/ 3/16" short diagonal
// Needs to be > 0.112" and < 0.1875, so 3.81 diameter
pub const STAND_XOVER_HOLE_DIAMETER: f64 = 3.81;
pub const STAND_XOVER_HOLE_RADIUS: f64 = STAND_XOVER_HOLE_DIAMETER / 2.0;
pub const STAND_XOVER_CENTER_Y: f64 = 225.0;

pub const XOVERCOVER_TOP_WIDTH: f64 = 225.0;
pub const XOVERCOVER_BOTTOM_WIDTH: f64 = 400.0;
pub const XOVERCOVER_HEIGHT: f64 = 230.0;

// COMPUTED PARAMETERS
pub const WOOFER_BOLT_CIRCLE_RADIUS: f64 = WOOFER_BOLT_CIRCLE_DIAMETER / 2.0;
pub const WOOFER_CUTOUT_RADIUS: f64 = WOOFER_CUTOUT_DIAMETER / 2.0;

pub const WOOFTRIM_OUTER_RADIUS: f64 = WOOFER_BOLT_CIRCLE_RADIUS + WOOFTRIM_THICKNESS / 2.0;
pub const WOOFTRIM_OUTER_DIAMETER: f64 = WOOFTRIM_OUTER_RADIUS * 2.0;
pub const WOOFTRIM_CUTOUT_RADIUS: f64 = WOOFER_BOLT_CIRCLE_RADIUS - WOOFTRIM_THICKNESS / 2.0;
pub const WOOFTRIM_BOLT_HOLE_RADIUS: f64 = WOOFTRIM_BOLT_HOLE_DIAMETER / 2.0;

pub const BAFFLE_CENTER_X: f64 = BAFFLE_WIDTH / 2.0;
pub const BAFFLE_LOWER_WOOF_CENTER_Y: f64 = BAFFLE_BOTTOM_GAP_Y + WOOFTRIM_OUTER_RADIUS;
pub const BAFFLE_UPPER_WOOF_CENTER_Y: f64 =
    BAFFLE_LOWER_WOOF_CENTER_Y + BAFFLE_WOOF_WOOF_GAP_Y + WOOFTRIM_OUTER_DIAMETER;

pub const WOOFGASKET_BOLT_HOLE_DIAMETER: f64 = WOOFTRIM_BOLT_HOLE_DIAMETER;
pub const WOOFGASKET_OUTER_RADIUS: f64 = WOOFER_BOLT_CIRCLE_RADIUS + WOOFGASKET_THICKNESS / 2.0;
pub const WOOFGASKET_CUTOUT_RADIUS: f64 = WOOFER_BOLT_CIRCLE_RADIUS - WOOFGASKET_THICKNESS / 2.0;
pub const WOOFGASKET_BOLT_HOLE_RADIUS: f64 = WOOFGASKET_BOLT_HOLE_DIAMETER / 2.0;

pub const FR_CUTOUT_RADIUS: f64 = FR_CUTOUT_DIAMETER / 2.0;
pub const FR_BOLT_CIRCLE_RADIUS: f64 = FR_BOLT_CIRCLE_DIAMETER / 2.0;
pub const FRTRIM_HEIGHT: f64 = 2.0 * FRTRIM_PADDING_Y + FR_FR_GAP + 2.0 * FR_OUTER_LENGTH;
pub const FRTRIM_WIDTH: f64 = 2.0 * FRTRIM_PADDING_X + FR_OUTER_LENGTH;

pub const FRTRIM_LL_X: f64 = (BAFFLE_WIDTH - FRTRIM_WIDTH) / 2.0;
pub const FRTRIM_LL_Y: f64 =
    BAFFLE_UPPER_WOOF_CENTER_Y + WOOFTRIM_OUTER_RADIUS + BAFFLE_WOOF_FR_GAP_Y;

pub const FRTRIM_UR_X: f64 = FRTRIM_LL_X + FRTRIM_WIDTH;
pub const FRTRIM_UR_Y: f64 = FRTRIM_LL_Y + FRTRIM_HEIGHT;
pub const FRTRIM_CUTOUT_RADIUS: f64 = (FR_CUTOUT_DIAMETER - 5.0) / 2.0;

pub const FRTRIM_BOLT_HOLE_RADIUS: f64 = BAFFLE_FR_BOLT_HOLE_DIAMETER / 2.0;

pub const FRGASKET_WIDTH: f64 = FRTRIM_WIDTH - 15.0;
pub const FRGASKET_HEIGHT: f64 = FRTRIM_HEIGHT - 15.0;
pub const FRGASKET_CUTOUT_RADIUS: f64 = (FR_CUTOUT_DIAMETER + 6.0) / 2.0;

pub const BAFFLE_HEIGHT: f64 = FRTRIM_UR_Y + BAFFLE_TOP_GAP_Y;

pub const BAFFLE_LOWER_FR_CENTER_Y: f64 = FRTRIM_LL_Y + FRTRIM_PADDING_Y + FR_OUTER_LENGTH / 2.0;
pub const BAFFLE_UPPER_FR_CENTER_Y: f64 = BAFFLE_LOWER_FR_CENTER_Y + FR_FR_GAP + FR_OUTER_LENGTH;

pub const BAFFLE_BEND_0_X: f64 = (BAFFLE_WIDTH - BAFFLE_CENTER_WIDTH) / 2.0;

pub const BAFFLE_LEFT_STAND_HOLE_CENTER_X: f64 = BAFFLE_BEND_0_X / 2.0;
pub const BAFFLE_LL_STAND_HOLE_CENTER_X: f64 =
    BAFFLE_LEFT_STAND_HOLE_CENTER_X - BAFFLE_STAND_HOLE_OFFSET_X;
pub const BAFFLE_UL_STAND_HOLE_CENTER_X: f64 =
    BAFFLE_LEFT_STAND_HOLE_CENTER_X + BAFFLE_STAND_HOLE_OFFSET_X;

pub const BAFFLE_RIGHT_STAND_HOLE_CENTER_X: f64 = BAFFLE_WIDTH - BAFFLE_BEND_0_X / 2.0;
pub const BAFFLE_LR_STAND_HOLE_CENTER_X: f64 =
    BAFFLE_RIGHT_STAND_HOLE_CENTER_X + BAFFLE_STAND_HOLE_OFFSET_X;
pub const BAFFLE_UR_STAND_HOLE_CENTER_X: f64 =
    BAFFLE_RIGHT_STAND_HOLE_CENTER_X - BAFFLE_STAND_HOLE_OFFSET_X;

// Thickness of stand attachment bar. Needs to be wide enough to cover pills.
pub const STAND_BAR_THICKNESS: f64 = (BAFFLE_LL_STAND_HOLE_CENTER_X
    + BAFFLE_STAND_HOLE_LENGTH / 2.0
    + BAFFLE_STAND_HOLE_THICKNESS / 2.0)
    - (BAFFLE_UL_STAND_HOLE_CENTER_X
        - BAFFLE_STAND_HOLE_LENGTH / 2.0
        - BAFFLE_STAND_HOLE_THICKNESS / 2.0)
    + 20.0;

// Must match the distance between the baffle stand hole x-axes post-bend.
pub fn stand_center_spine_length() -> f64 {
    BAFFLE_CENTER_WIDTH + BAFFLE_BEND_0_X * f64::cos(BAFFLE_SIDE_ANGLE)
}

pub const STANDGASKET_PADDING: f64 = 15.0;
pub const STANDGASKET_WIDTH: f64 = STAND_BAR_THICKNESS-STANDGASKET_PADDING;
pub const STANDGASKET_HEIGHT: f64 = STAND_WING_LENGTH-STANDGASKET_PADDING;


