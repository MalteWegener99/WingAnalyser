use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Dataset{
    pub y: f64,
    pub shear: f64,
    pub moment: f64,
    pub torsion: f64,
}

pub fn parse(location: &str) -> Vec<Dataset>{
    let mut vc: Vec<Dataset> = Vec::new();

    let file = File::open(location);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents);
    let split: Vec<String> = contents.split("\r\n").map(|s| s.to_string()).collect();
    for s in split{
        let split2 = s.split(" ").filter_map(|s| s.parse::<f64>().ok()).collect::<Vec<_>>();
        if split2.len() > 0{
            vc.push(Dataset{
                y: split2[0],
                shear: split2[1]*1.5,
                moment: split2[3]*1.5,
                torsion: split2[5]*1.5,
            })
        }
    }

    vc
}