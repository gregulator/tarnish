// Run with: cargo run

// TODO:
// - Set units to MM (Done but SCS still checks)
// - Bendlines
// - Rounded rectangles
// - Bolt hole rings
// - Gen handles
mod dxf;
mod geom;

// use std::fs::File;
// use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    //let p = Part {
    //    outline: Shape::Rect(Rect {
    //        ll: Vec2 { x: 0.0, y: 0.0 },
    //        ur: Vec2 { x: 100.0, y: 100.0 },
    //    }),
    //    cutouts: Vec::new(),
    //    bendlines: Vec::new(),
    //};
    //let serialized = serde_json::to_string(&p).unwrap();
    // let mut file = File::create("out.dxf")?;
    // write!(file, "{}{}", dxf::HEADER, serialized)?;
    //println!("serialized = {}", serialized);
    let extent = geom::Bounds2 {
        min: geom::Vec2 { x: -200.0, y: -200.0 },
        max: geom::Vec2 {
            x: 200.0,
            y: 200.0,
        },
    };
    println!("{}", dxf::gen_header(extent));
    println!("{}", dxf::TABLES);
    println!("{}", dxf::BLOCKS);
    println!("{}", dxf::ENTITIES_HEADER);
    println!("{}", dxf::gen_circle(geom::Circle{
      center: geom::Vec2{x: 0.0, y: 0.0},
      radius: 200.0,
    }));
    println!("{}", dxf::gen_rect(geom::Rect{
      ll: geom::Vec2{x: 0.0, y: 0.0},
      ur: geom::Vec2{x: 40.5, y: 60.5},
    }));
    println!("{}", dxf::gen_bendline(geom::LineSeg{
      p0: geom::Vec2{x: 0.0, y: -200.0},
      p1: geom::Vec2{x: 50.0, y: 200.0},
    }));
    //println!("{}", dxf::gen_circle(geom::Circle{
    //  center: geom::Vec2{x: 0.0, y: 0.0},
    //  radius: 100.0,
    //}));
    println!("{}", dxf::ENTITIES_FOOTER);
    println!("{}", dxf::FOOTER);
    //println!("{}", dxf::gen_header(extent));
    //println!("{}", dxf::EMPTY2);

    Ok(())
}
