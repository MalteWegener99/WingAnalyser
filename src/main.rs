mod Vector;
use Vector::Vec2;
mod section;
use section::section_prototype;
use std::time::{Duration, Instant};
mod optimizer;
use optimizer::optimize;

fn main() {
    let vertices = [Vec2{x: 0., y: 0.},Vec2{x: 9., y: 0.},Vec2{x: 9., y: 2.},Vec2{x: 0., y: 2.}];
    let section = optimize(0., 3e6, 5e7, 0.0001, vertices).generateanalysable();
    let maxes = section.get_max_stress(0., 3e6, 5e7);
    for i in 0..4{
        let mut name = "";
        match i{
            0 => name = "Lower skin",
            1 => name = "Rear spar",
            2 => name = "Upper skin",
            3 => name = "Front Spar",
            _ => name = "Index out of bound error"
        }
        println!("{}: \t tau.max: {:.1} MPa, \t sigma.max {:.1} MPa", name, maxes.0[i]/1000000., maxes.1[i]/1000000.);
    }
    println!("{:.5}, {:.5}, {:.5}, {:.5}", section.skin_thicknesses[0],section.skin_thicknesses[1],section.skin_thicknesses[2],section.skin_thicknesses[3]);
    println!("{}, {}", section.stringers[0], section.stringers[1])
}
