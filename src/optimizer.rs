use Vector::Vec2;
use section::section_prototype;
use std::u16;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use std::io::LineWriter;

pub fn optimize(torque: f64, shear: f64, moment: f64, factor: f64, verti: [Vec2; 4], minstr: u16) -> section_prototype{
    let taumax: f64 = 207000000.;
    let sigmax: f64 = 276000000.;

    /*let mut logfile = File::create("Optimization.log");
    let mut logfile = match logfile {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    let mut logfile = LineWriter::new(logfile);*/
    //first iteration
    let mut section = section_prototype{
        vertices: verti,
        skin_thicknesses: [0.0001; 4],
        stringers: [minstr; 2],
        area_stringer: 3000e-6,
    };

    let mut done = false;
    let mut counter = 0;

    while !done{
        counter += 1;
        let mut skin_ = section.skin_thicknesses;
        let mut strs_ = section.stringers;
        let astr_ = section.area_stringer;

        let generated = section.generateanalysable();
        let maxes = generated.get_max_stress(torque, shear, moment);

        //breakoutlogic
        let rotation = generated.get_torsion_per_length(torque)*0.1625;
        let mut ct = 0;
        for i in 0..4{
            if maxes.0[i].abs() <= taumax*1.05{
                ct += 1;
            }
            else{
                //logfile.write_all(format!("{} tau not succeded ({:.2}, {:.2})/", i, maxes.0[i]/taumax, maxes.0[i]/1000000.).as_bytes());
            }
        }
        for i in 0..2{
            if maxes.1[i*2].abs() <= sigmax*1.05 {
                ct += 1;
            }
            else{
                //logfile.write_all(format!("{} sig not succeded ({:.2}, {:.2})/", i, maxes.1[i*2]/sigmax, maxes.1[i*2]/1000000.).as_bytes());
            }
        }

        if rotation <= 8.72e-4{
            ct += 1;
        }

        //logfile.write_all(format!("{} {} {}\r\n", counter, ct, generated.get_weight_per_len(2700.)).as_bytes());
        if ct >= 7{
            done = true;
        }
        else{
            //optimization logic
            //the 1.1 is for stable convergance
            for i in 0..4{
                if rotation*1.1 >= 8.72e-4 || maxes.0[i].abs()*1.15 >= taumax{
                    skin_[i] += factor;
                }
                else if skin_[i] > (2.*factor){
                    skin_[i] -= factor;
                }
            }

            for i in 0..2{
                if maxes.1[i*2].abs()*1.15 >= sigmax{
                    strs_[i] += 1;
                }
                else if strs_[i] > minstr{
                    strs_[i] -= 1;
                }
            }
            section = section_prototype{
                vertices: verti,
                skin_thicknesses: skin_,
                stringers: strs_,
                area_stringer: astr_,
            };
        }
    }
    println!("Converged in {} iterations", counter);
    section
}
