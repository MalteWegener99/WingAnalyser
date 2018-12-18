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
use std::string;
mod triangle;
use triangle::{Triangle, Vec3, write_stl};
mod stl;
use stl::make_stl;


fn chord(y: f64) -> f64{
    if y<=12.2{
        return 20.44-(20.44-10.02)/12.2*y;
    
    }
    else{
        return 10.02-(10.02-5.52)/(32.5-12.2)*(y-12.2);
    }
}

fn chord2(y: f32) -> f32{
    chord(y as f64) as f32
}

fn chordlen(y: f64) -> f64{
    10.02-(10.02-5.52)/(32.5-12.2)*(y-12.2)
}

fn analyse(file: &str, minstr: u16, flag: bool) -> Vec<([f64; 4], [u16; 2], [f64; 4], [f64; 4])>{
    let d: Vec<Dataset> = parse(file)
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
        let vertices = if flag{
            [Vec2{x: 0., y: 0.}.scale(chord(thing.y)),Vec2{x: 0.45, y: 0.0182}.scale(chord(thing.y)),Vec2{x: 0.45, y: 0.1051}.scale(chord(thing.y)),Vec2{x: 0., y: 0.1092}.scale(chord(thing.y))]
        }
        else{
            [Vec2{x: 0., y: 0.},Vec2{x: 0.45*chordlen(thing.y), y: 0.0182*chord(thing.y)},Vec2{x: 0.45*chordlen(thing.y), y: 0.1051*chord(thing.y)},Vec2{x: 0., y: 0.1092*chord(thing.y)}]
        };
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
    results
}

trait discretize{
    fn discretize(&self, sections: &Vec<f64>) -> Vec<(f64, [f64; 4], [u16; 2])>;
}

trait save{
    fn save(&self, file: &str) -> Result<(),io::Error>;
}

impl save for Vec<(f64, [f64; 4], [u16; 2])>{
    fn save(&self, file: &str) -> Result<(),io::Error>{
        let mut output = File::create(file)?;
        output.write_all("y,LowerSkin,RearSpar,UpperSkin,FrontSpar,StringersBot,StringersTop\r\n".as_bytes())?;
        for thing in self{
            output.write_all(format!("{},{},{},{},{},{},{}\r\n", thing.0, thing.1[0], thing.1[1], thing.1[2], thing.1[3], thing.2[0], thing.2[1]).as_bytes())?;
        }

        Ok(())
    }
}

impl discretize for Vec<([f64; 4], [u16; 2], [f64; 4], [f64; 4])>{
    fn discretize(&self, sections: &Vec<f64>) -> Vec<(f64, [f64; 4], [u16; 2])>{
        let lowersurface: Vec<f64> = self.into_iter().map(|x| x.0[0]).collect();
        let rearspar: Vec<f64> = self.into_iter().map(|x| x.0[1]).collect();
        let uppersurface: Vec<f64> = self.into_iter().map(|x| x.0[2]).collect();
        let frontspar: Vec<f64> = self.into_iter().map(|x| x.0[3]).collect();

        let lowerstr: Vec<u16> = self.into_iter().map(|x| x.1[0]).collect();
        let upperstr: Vec<u16> = self.into_iter().map(|x| x.1[1]).collect();

        //now lets find the niggershit section seperators
        let dy = 0.1625;
        let splits: Vec<usize> = sections.into_iter()
                                            .map(|x| (x/dy) as usize)
                                            .collect::<Vec<usize>>();


        //Ok this is less rustic now. Man i fucking love functional programming
        let mut lowersurfacemax: Vec<f64> = Vec::with_capacity(splits.len()+1).into_iter().map(|x: f64| 0.).collect();
        let mut rearsparmax: Vec<f64> = Vec::with_capacity(splits.len()+1).into_iter().map(|x: f64| 0.).collect();
        let mut uppersurfacemax: Vec<f64> = Vec::with_capacity(splits.len()+1).into_iter().map(|x: f64| 0.).collect();
        let mut frontsparmax: Vec<f64> = Vec::with_capacity(splits.len()+1).into_iter().map(|x: f64| 0.).collect();

        let mut lowerstrmax: Vec<u16> = Vec::with_capacity(splits.len()+1).into_iter().map(|x: u16| 0).collect();
        let mut upperstrmax: Vec<u16> = Vec::with_capacity(splits.len()+1).into_iter().map(|x: u16| 0).collect();
        for i in 0..splits.len()+1{
            lowersurfacemax.push(0.);
            rearsparmax.push(0.);
            uppersurfacemax.push(0.);
            frontsparmax.push(0.);

            lowerstrmax.push(0);
            upperstrmax.push(0);
        }

        for i in 0..lowersurface.len(){
            for j in 0..splits.len(){
                if i <= splits[j]{
                    lowersurfacemax[j] = lowersurfacemax[j].max(lowersurface[i]);
                    rearsparmax[j] = rearsparmax[j].max(rearspar[i]);
                    uppersurfacemax[j] = uppersurfacemax[j].max(uppersurface[i]);
                    frontsparmax[j] = frontsparmax[j].max(frontspar[i]);

                    lowerstrmax[j] = lowerstrmax[j].max(lowerstr[i]);
                    upperstrmax[j] = upperstrmax[j].max(upperstr[i]);
                    break;
                }
            }
            if i > splits[splits.len()-1]{
                lowersurfacemax[splits.len()] = lowersurfacemax[splits.len()].max(lowersurface[i]);
                rearsparmax[splits.len()] = rearsparmax[splits.len()].max(rearspar[i]);
                uppersurfacemax[splits.len()] = uppersurfacemax[splits.len()].max(uppersurface[i]);
                frontsparmax[splits.len()] = frontsparmax[splits.len()].max(frontspar[i]);

                lowerstrmax[splits.len()] = lowerstrmax[splits.len()].max(lowerstr[i]);
                upperstrmax[splits.len()] = upperstrmax[splits.len()].max(upperstr[i]);
            }
        }

        let mut ret = Vec::with_capacity(lowersurfacemax.len());

        for i in 0..lowersurfacemax.len(){
            let y =  if i > 0 {
                sections[i-1]
            }else{ 
                0.
            };
            ret.push((y, [lowersurfacemax[i],rearsparmax[i],uppersurfacemax[i],frontsparmax[i]], [lowerstrmax[i],upperstrmax[i]]));
        }

        ret
    }
}


fn main(){

    let vecci1 = Vec3{x: 0., y: 0., z: 0.};
    let vecci2 = Vec3{x: 1., y: -1., z: 0.};
    let vecci3 = Vec3{x: 1., y: 1., z: 0.};
    let mut vc: Vec<Triangle> = Vec::new();
    vc.push(Triangle::new(&vecci1, &vecci2, &vecci3));
    vc.push(Triangle::new(&vecci1, &vecci2, &vecci3.scale(-1.)));
    vc.push(Triangle::new(&vecci1, &vecci2.scale(-1.), &vecci3.scale(-1.)));
    vc.push(Triangle::new(&vecci1, &vecci2.scale(-1.), &vecci3));
    write_stl("test.stl", &vc);
    make_stl((40.01 as f32).to_radians(), &chord2, 32.5);

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
    let stringersinput = trimmed.parse::<u16>().unwrap();

    println!("Enter the spanwise positions where you want to split don't enter 0 or the halfspan you donkey(Enter n to proceed):");
    let mut sections: Vec<f64> = Vec::new();
    while true{
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        if trimmed == "N" || trimmed == "n"{
            break;
        }
        match trimmed.parse::<f64>() {
            Ok(i) => print!(""),
            Err(..) => {println!("this was not a float: {}", trimmed); continue;},
        };
        sections.push(trimmed.parse::<f64>().unwrap());

    }

    println!("Which wingboxstyle do you want to use? 0 for kinked wingbox, 1: for straight wingbox:");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => println!("Good input"),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    let flaggy = trimmed.parse::<i32>().unwrap();

    analyse("output.csv", stringersinput, flaggy == 0).discretize(&vec![12.2, 20., 25., 30.]).save("Design.csv");
    analyse("output-1.csv", stringersinput, flaggy != 0).discretize(&vec![12.2, 20., 25., 30.]).save("Design-1.csv");
}