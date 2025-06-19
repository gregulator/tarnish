//mod geom;

use crate::geom;

pub fn gen_header(extent: geom::Bounds2) -> std::string::String {
    return format!(
        "   999
CREATED BY GREG PRISAMENT USING TARNISH (https://github.com/gregulator/tarnish)
  999
--- HEADER SECTION ---
  0
SECTION
  2
HEADER
  9
$ACADVER
  1
AC1006
  9
$INSBASE
  10
0.0
  20
0.0
  30
0.0
  9
$EXTMIN
  10
{:.1}
  20
{:.1}
  30
0
  9
$EXTMAX
  10
{:.1}
  20
{:.1}
  30
0
  0
ENDSEC",
        extent.min.x, extent.min.y, extent.max.x, extent.max.y
    );
}

pub const ENTITIES_HEADER: &str = "999
--- ENTITIES SECTION ---
0
SECTION
2
ENTITIES";

pub fn gen_circle(circle: geom::Circle) -> std::string::String {
    return format!(
  "   999
--- CIRLCE AT ({:.3}, {:.3}) with radius {} ---
  0
CIRCLE
  5
4E
  100
AcDbEntity
  8
0
  6
ByLayer
 62
256
  370
-1
  100
AcDbCircle
 10
{:.3}
 20
{:.3}
 40
{:.3}", circle.center.x, circle.center.y, circle.radius, circle.center.x, circle.center.y, circle.radius);
}

pub const ENTITIES_FOOTER: &str = "  0
ENDSEC";

// Table section. Currently has 1 line type and 1 layer.
pub const TABLES: &str = "999
--- TABLES SECTION ---
0
SECTION
2
TABLES
0
TABLE
2
LTYPE
70
1
0
LTYPE
2
CONTINUOUS
70
64
3
Solid line
72
65
73
0
40
0.000000
0
ENDTAB
0
TABLE
2
LAYER
70
6
0
LAYER
2
1
70
64
62
7
6
CONTINUOUS
0
ENDTAB
0
ENDSEC";

pub const FOOTER: &str = "999
--- END SECTION ---
0
EOF";

pub const BLOCKS: &str = "999
--- BLOCKS SECTION ---
0
SECTION
2
BLOCKS
0
ENDSEC";

pub const ENTITIES: &str = "999
--- ENTITIES SECTION ---
0
SECTION
2
ENTITIES
0
ENDSEC";

pub const ENTITIES_WITH_CIRCLE: &str = "999
--- ENTITIES SECTION W/ CIRCLE---
0
SECTION
2
ENTITIES
  0
CIRCLE
  5
4E
  100
AcDbEntity
  8
0
  6
ByLayer
 62
256
  370
-1
  100
AcDbCircle
 10
80.0
 20
90.0
 40
20.0
  0
ENDSEC";
