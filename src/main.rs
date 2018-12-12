mod Vector;
use Vector::Vec2;
mod section;
use section::section_prototype;
use std::time::{Duration, Instant};
mod optimizer;
use optimizer::optimize;
mod parsing;
use parsing::Dataset;
use parsing::parse;

fn main() {
    let d = parse("C:/Users/malte/Documents/Task44/output.csv");
    let now = std::time::Instant::now();
    let mut counter: u32 = 0;
    let mut weight = 0.;
    println!("{:?}", d);
    for thing in &d{
        counter += 1;
        println!("{}", counter);
        let vertices = [Vec2{x: 0., y: 0.},Vec2{x: 9.-9.*thing.y/32.5, y: 0.},Vec2{x: 9.-9.*thing.y/32.5, y: 2.-2.*thing.y/32.5},Vec2{x: 0., y: 3.-3.*thing.y/32.5}];
        let section = optimize(thing.torsion, thing.shear, thing.moment, 0.00001, vertices).generateanalysable();
        let maxes = section.get_max_stress(thing.torsion, thing.shear, thing.moment);
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
        weight += section.get_weight_per_len(2700.)*64./400.;
        println!("Mass per len:{}", section.get_weight_per_len(2700.));
        println!("{:.2}, {:.2}, {:.2}, {:.2}", section.skin_thicknesses[0]*1000.,section.skin_thicknesses[1]*1000.,section.skin_thicknesses[2]*1000.,section.skin_thicknesses[3]*1000.);
        println!("{}, {}", section.stringers[0], section.stringers[1])
    }
    println!("I took {} minutes", (now.elapsed().as_secs() as f64)/100.);
    println!("And i weigh {} kg", weight);
}
