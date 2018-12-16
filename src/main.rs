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
use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use std::io::LineWriter;
use std::io;

fn chord(y: f64) -> f64{
    if y<=12.2{
        return 20.44-(20.44-10.02)/12.2*y;
    
    }
    else{
        return 10.02-(10.02-5.52)/(32.5-12.2)*(y-12.2);
    }
}


fn main() {
    println!("Enter the minumum amount of stringers:");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("Good input"),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    let stringersinput = trimmed.parse::<u32>().unwrap();

    let d: Vec<Dataset> = parse("output.csv")
                            .into_iter()
                            .filter(|&thing| thing.shear != 0.)
                            .collect::<Vec<Dataset> >();
    let now = std::time::Instant::now();
    let mut counter: u32 = 0;
    let mut weight = 0.;

    let mut results: Vec<([f64; 4], [u16; 2], [f64; 4], [f64; 4])> = Vec::new();
    println!("{:?}", d);
    for thing in &d{
        counter += 1;
        println!("{}", counter);
        let vertices = [Vec2{x: 0., y: 0.}.scale(chord(thing.y)),Vec2{x: 0.45, y: 0.0182}.scale(chord(thing.y)),Vec2{x: 0.45, y: 0.1051}.scale(chord(thing.y)),Vec2{x: 0., y: 0.1092}.scale(chord(thing.y))];
        let section = optimize(thing.torsion, thing.shear, thing.moment, 0.00001, vertices, 30).generateanalysable();
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
        weight += section.get_weight_per_len(2700.)*0.1625;

        results.push((section.skin_thicknesses,section.stringers, maxes.0, maxes.1));

        println!("Mass per len:{:.2}", section.get_weight_per_len(2700.));
        println!("{:.2}, {:.2}, {:.2}, {:.2}", section.skin_thicknesses[0]*1000.,section.skin_thicknesses[1]*1000.,section.skin_thicknesses[2]*1000.,section.skin_thicknesses[3]*1000.);
        println!("{}, {}", section.stringers[0], section.stringers[1])
    }
    println!("I took {} seconds", (now.elapsed().as_secs() as f64));
    println!("And i weigh {} kg", weight);

    let mut output = File::create("Design.csv");
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let mut output = LineWriter::new(output);
    for res in &results{
        output.write_all(format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{}\r\n", res.0[0], res.0[1], res.0[2], res.0[3], res.1[0], res.1[1],
                                                                                    res.2[0],res.2[1],res.2[2],res.2[3],res.3[0],res.3[1],res.3[2],res.3[3]).as_bytes());
    }
    println!("saved1");

    let d = parse("output-1.csv")
                            .into_iter()
                            .filter(|&thing| thing.shear != 0.)
                            .collect::<Vec<Dataset> >();
    let now = std::time::Instant::now();
    let mut counter: u32 = 0;
    let mut weight = 0.;

    let mut results: Vec<([f64; 4], [u16; 2], [f64; 4], [f64; 4])> = Vec::new();
    println!("{:?}", d);
    for thing in &d{
        counter += 1;
        println!("{}", counter);
        let vertices = [Vec2{x: 0., y: 0.}.scale(chord(thing.y)),Vec2{x: 0.45, y: 0.0182}.scale(chord(thing.y)),Vec2{x: 0.45, y: 0.1051}.scale(chord(thing.y)),Vec2{x: 0., y: 0.1092}.scale(chord(thing.y))];
        let section = optimize(thing.torsion, thing.shear, thing.moment, 0.00001, vertices, 30).generateanalysable();
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
        weight += section.get_weight_per_len(2700.)*0.1625;

        results.push((section.skin_thicknesses,section.stringers, maxes.0, maxes.1));

        println!("Mass per len:{:.2}", section.get_weight_per_len(2700.));
        println!("{:.2}, {:.2}, {:.2}, {:.2}", section.skin_thicknesses[0]*1000.,section.skin_thicknesses[1]*1000.,section.skin_thicknesses[2]*1000.,section.skin_thicknesses[3]*1000.);
        println!("{}, {}", section.stringers[0], section.stringers[1])
    }
    println!("I took {} seconds", (now.elapsed().as_secs() as f64));
    println!("And i weigh {} kg", weight);

    let mut output = File::create("Design-1.csv");
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let mut output = LineWriter::new(output);
    for res in &results{
        output.write_all(format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{}\r\n", res.0[0], res.0[1], res.0[2], res.0[3], res.1[0], res.1[1],
                                                                                    res.2[0],res.2[1],res.2[2],res.2[3],res.3[0],res.3[1],res.3[2],res.3[3]).as_bytes());
    }
    println!("saved2")
}
