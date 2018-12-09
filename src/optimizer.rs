use Vector::Vec2;
use section::section_prototype;
use std::u16;

pub fn optimize(torque: f64, shear: f64, moment: f64, factor: f64, verti: [Vec2; 4]) -> section_prototype{
    let taumax = 207e6;
    let sigmax = 386e6;

    println!("{}, {}", taumax, sigmax);
    //first iteration
    let mut section = section_prototype{
        vertices: verti,
        skin_thicknesses: [0.001; 4],
        stringers: [2; 2],
        area_stringer: 740e-6,
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
        let mut ct = 0;
        for i in 0..4{
            if maxes.0[i].abs() < taumax{
                ct += 1;
            }
        }
        for i in 0..2{
            if maxes.1[i*2].abs() < sigmax{
                ct += 1;
            }
        }
        if ct >= 6{
            done = true;
        }

        //optimization logic
        //first the shear(thats maxes.0)
        for i in 0..4{
            if maxes.0[i].abs() > taumax{
                skin_[i] += factor;
            }
            else if skin_[i] > (2.*factor){
                skin_[i] -= factor;
            }
        }

        for i in 0..2{
            if maxes.1[i*2].abs() > sigmax{
                strs_[i] += 1;
            }
            else{
                if strs_[i] >= 2{
                    strs_[i] -= 1;
                }
            }
        }

        section = section_prototype{
            vertices: verti,
            skin_thicknesses: skin_,
            stringers: strs_,
            area_stringer: astr_,
        };
    }

    section
}