extern crate byteorder;
use self::byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use std::io::LineWriter;
use std::io;
use std::string;

#[derive(Debug, Clone, Copy)]
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3{
    pub fn add(&self, other: &Vec3) -> Vec3{
        Vec3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn scale(&self, other: f32) -> Vec3{
        Vec3{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }

    pub fn sub(&self, other: &Vec3) -> Vec3{
        self.add(&other.scale(-1.))
    }

    pub fn cross(&self, other: &Vec3) -> Vec3{
        Vec3{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn invertz(&self) -> Vec3{
        Vec3{
            x: self.x,
            y: self.y,
            z: self.z * -1.,
        }
    }

    pub fn magnitude(&self) -> f32{
        (self.x.powi(2)+self.y.powi(2)+self.z.powi(2)).sqrt()
    }

    pub fn unitvec(&self) -> Vec3{
        self.scale(1./self.magnitude())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle{
    pub n: Vec3,
    pub i: Vec3,
    pub j: Vec3,
    pub k: Vec3,
}

impl Triangle{
    pub fn new(one: &Vec3, two: &Vec3, three: &Vec3) -> Triangle{
        Triangle{
            n: two.sub(one).cross(&three.sub(one)),
            i: *one,
            j: *two,
            k: *three,
        }
    }

    pub fn invertn(&self) -> Triangle{
        Triangle{
            n: self.n.scale(-1.),
            i: self.i,
            j: self.j,
            k: self.k,
        }
    }

    pub fn as_bytes(&self) -> [u8; 50]{
        let mut buffer: [u8; 4] = [0; 4];
        let mut vecbuf: Vec<u8> = Vec::with_capacity(50);
        let mut finalbuf: [u8; 50] = [0; 50];

        //normal vector
        LittleEndian::write_f32(&mut buffer, self.n.x);
        vecbuf.extend_from_slice(&buffer);
        LittleEndian::write_f32(&mut buffer, self.n.y);
        vecbuf.extend_from_slice(&buffer);
        LittleEndian::write_f32(&mut buffer, self.n.z);
        vecbuf.extend_from_slice(&buffer);
        
        LittleEndian::write_f32(&mut buffer, self.i.x);
        vecbuf.extend_from_slice(&buffer);
        LittleEndian::write_f32(&mut buffer, self.i.y);
        vecbuf.extend_from_slice(&buffer);
        LittleEndian::write_f32(&mut buffer, self.i.z);
        vecbuf.extend_from_slice(&buffer);
        
        LittleEndian::write_f32(&mut buffer, self.j.x);
        vecbuf.extend_from_slice(&buffer);
        LittleEndian::write_f32(&mut buffer, self.j.y);
        vecbuf.extend_from_slice(&buffer);
        LittleEndian::write_f32(&mut buffer, self.j.z);
        vecbuf.extend_from_slice(&buffer);

        LittleEndian::write_f32(&mut buffer, self.k.x);
        vecbuf.extend_from_slice(&buffer);
        LittleEndian::write_f32(&mut buffer, self.k.y);
        vecbuf.extend_from_slice(&buffer);
        LittleEndian::write_f32(&mut buffer, self.k.z);
        vecbuf.extend_from_slice(&buffer);

        vecbuf.extend_from_slice(&[0, 0]);

        for i in 0..50{
            finalbuf[i] = vecbuf[i];
        }

        finalbuf
    }
}

pub fn write_stl(file: &str, triangles: &Vec<Triangle>) -> Result<(), std::io::Error>{
    let mut out = File::create(file)?;
    out.write_all(&[0xFF; 80])?;
    let mut sizebuf = [0; 4];
    LittleEndian::write_u32(&mut sizebuf, triangles.len() as u32);
    out.write_all(&sizebuf)?;
    for t in triangles{
        out.write_all(&t.as_bytes())?;
    }
    out.flush()?;

    Ok(())
}