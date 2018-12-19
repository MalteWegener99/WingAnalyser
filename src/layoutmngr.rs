use Vector::{Vec2};

fn chord(y: f64) -> f64{
    if y<=12.2{
        return 20.44-(20.44-10.02)/12.2*y;
    
    }
    else{
        return 10.02-(10.02-5.52)/(32.5-12.2)*(y-12.2);
    }
}

//has to have a section at the kink
pub fn generate_layout(sections: &Vec<(f64, [f64; 4], [u16; 2])>) -> [Vec<(f32, f32)>; 2]{
    let fspar = 0.2;
    let rspar = 0.65;
    let sweep = (40.01 as f64).to_radians();
    let corners = [
        Vec2{x: 0., y: fspar * chord(0.)},
        Vec2{x: 12.2, y: fspar * chord(12.2) + 12.2*sweep.tan()},
        Vec2{x: 32.5, y: fspar * chord(32.5) + 32.5*sweep.tan()},
        Vec2{x: 32.5, y: rspar * chord(32.5) + 32.5*sweep.tan()},
        Vec2{x: 12.2, y: rspar * chord(12.2) + 12.2*sweep.tan()},
        Vec2{x: 0., y: rspar * chord(0.)},
    ];

    //(position chord, end)
    let mut stringsup: Vec<(f32, f32)> = Vec::new();
    let mut stringsdo: Vec<(f32, f32)> = Vec::new();

    for section in sections.into_iter().rev(){
        if (section.2[0] as usize) > stringsdo.len(){
            if stringsdo.len() == 0{
                for i in 0..section.2[0]{
                    stringsdo.push(((fspar as f32)+0.01 + (i as f32)*((rspar-fspar-0.02) as f32)/(section.2[0]as f32) , 32.5));
                }
            }
            else if (section.2[0] as usize)-stringsdo.len() < stringsdo.len(){
                for i in 0..stringsdo.len()-1{
                    let uno = stringsdo[i].0;
                    let dos = stringsdo[i+1].0;
                    stringsdo.push(((uno+dos)/2., 12.2));
                }
            }

            else if (section.2[0] as usize)-stringsdo.len() <= (stringsdo.len()-1)*2{
                for i in 0..stringsdo.len()-1{
                    let uno = stringsdo[i].0;
                    let dos = stringsdo[i+1].0;
                    stringsdo.push(((uno+dos)/3., 12.2));
                    stringsdo.push(((uno+dos)/3.*2., 12.2));
                }
            }
        }

        if (section.2[0] as usize) > stringsup.len(){
            if stringsup.len() == 0{
                for i in 0..section.2[0]{
                    stringsup.push(((fspar as f32)+0.01 + (i as f32)*((rspar-fspar-0.02) as f32)/(section.2[0]as f32) , 32.5));
                }
            }
            else if (section.2[0] as usize)-stringsup.len() < stringsup.len(){
                for i in 0..stringsup.len()-1{
                    let uno = stringsup[i].0;
                    let dos = stringsup[i+1].0;
                    stringsup.push(((uno+dos)/2., 12.2));
                }
            }

            else if (section.2[0] as usize)-stringsup.len() <= (stringsup.len()-1)*2{
                for i in 0..stringsup.len()-1{
                    let uno = stringsup[i].0;
                    let dos = stringsup[i+1].0;
                    stringsup.push(((uno+dos)/3., 12.2));
                    stringsup.push(((uno+dos)/3.*2., 12.2));
                }
            }
        }
    }

    [stringsdo, stringsup]
}