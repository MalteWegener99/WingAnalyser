use Vector::Vec2;
use std::vec;

#[derive(Debug, Copy, Clone)]
pub struct section_prototype{
    //the 4 vertices of the 
    pub vertices: [Vec2; 4],
    pub skin_thicknesses: [f64; 4],
    pub stringers: [u16; 2],
    pub area_stringer: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct xyt{
    pub x: f64,
    pub y: f64,
    pub t: f64,
}

impl section_prototype{
    fn centroid(&self) -> Vec2{
        //Calculate the total Area
        let mut area_total: f64 = (self.stringers.iter().sum::<u16>() as f64) * self.area_stringer;
        for i in 0..self.vertices.len(){
            area_total += self.skin_thicknesses[i] * self.vertices[i].substract(&self.vertices[(i+1)%self.vertices.len()]).magnitude();
        }

        //Calculate the centers of each Panel
        let mut centers = [Vec2{x: 0., y: 0.}; 4];
        let mut length: [f64; 4] = [0.; 4];

        for i in 0..self.vertices.len(){
            centers[i] = self.vertices[i].add(&self.vertices[(i+1)%4].substract(&self.vertices[i]).scale(0.5));
            length[i] = self.vertices[(i+1)%4].substract(&self.vertices[i]).magnitude();
        }

        //calculate Qx
        let mut qy = (self.stringers[0] as f64) * centers[0].y * self.area_stringer;
        qy += (self.stringers[1] as f64) * centers[2].y * self.area_stringer;

        let mut qx = (self.stringers[0] as f64) * centers[0].x * self.area_stringer;
        qx += (self.stringers[1] as f64) * centers[2].x * self.area_stringer;

        for i in 0..self.vertices.len(){
            qy += length[i]*self.skin_thicknesses[i] * centers[i].y;
            qx += length[i]*self.skin_thicknesses[i] * centers[i].x;
        }

        Vec2{x:qx/area_total, y:qy/area_total}
    }

    fn inertia(&self, centro: Vec2) -> (f64, f64, f64){

        //centers with respect to the centroid
        let mut centers = [Vec2{x: 0., y: 0.}; 4];
        let mut length: [f64; 4] = [0.; 4];

        for i in 0..self.vertices.len(){
            centers[i] = self.vertices[i].add(&self.vertices[(i+1)%4].substract(&self.vertices[i]).scale(0.5)).substract(&centro);
            length[i] = self.vertices[(i+1)%4].substract(&self.vertices[i]).magnitude();
        }

        //Calculate Ixx and Iyy and of course our favourite retarded child Ixy
        //spar contribution only for Ixx because i can
        let mut ixx = length[1].powi(3)*self.skin_thicknesses[1]/12.;
        ixx += length[3].powi(3)*self.skin_thicknesses[3]/12.;

        let mut iyy = 0.;
        let mut ixy = 0.;

        //steiner terms
        for i in 0..centers.len(){
            ixx += length[i]*self.skin_thicknesses[i]*centers[i].y.powi(2);
            iyy += length[i]*self.skin_thicknesses[i]*centers[i].x.powi(2);
            iyy += length[i]*self.skin_thicknesses[i]*centers[i].x*centers[i].y;
        }

        //and the stringers
        ixx += (self.stringers[0] as f64) * self.area_stringer * centers[0].y.powi(2);
        ixx += (self.stringers[1] as f64) * self.area_stringer * centers[2].y.powi(2);

        iyy += (self.stringers[0] as f64) * self.area_stringer * centers[0].x.powi(2);
        iyy += (self.stringers[1] as f64) * self.area_stringer * centers[2].x.powi(2);

        ixy += (self.stringers[0] as f64) * self.area_stringer * centers[0].y * centers[0].x;
        ixy += (self.stringers[1] as f64) * self.area_stringer * centers[2].y * centers[2].x;
        /*println!("Ixx: {}", ixx);
        println!("Iyy: {}", iyy);
        println!("Ixy: {}", ixy);*/

        (ixx, iyy, ixy)
    }

    fn xytmap(&self, centro: Vec2) -> (Vec<xyt>, [usize; 5], f64){
        let mut vc = Vec::new();

        //centers with respect to the centroid
        let mut centers = [Vec2{x: 0., y: 0.}; 4];
        let mut length: [f64; 4] = [0.; 4];
        let mut unitvecs = [Vec2{x: 0., y: 0.}; 4];

        for i in 0..self.vertices.len(){
            centers[i] = self.vertices[i].add(&self.vertices[(i+1)%4].substract(&self.vertices[i]).scale(0.5)).substract(&centro);
            length[i] = self.vertices[(i+1)%4].substract(&self.vertices[i]).magnitude();
            unitvecs[i] = self.vertices[(i+1)%4].substract(&self.vertices[i]).unitvec();
        }

        let subdivs = 1000;

        let ds: f64 = length.iter().sum::<f64>()/(subdivs as f64);
        let mut corners: [usize; 5] = [0; 5];

        //pretty ugly implementation
        corners[1] = (length[0]/ds).round() as usize;
        corners[2] = ((length[0]+length[1])/ds).round() as usize;
        corners[3] = ((length[0]+length[1]+length[2])/ds).round() as usize;
        corners[4] = (subdivs-1) as usize;

        for i in 0..subdivs{
            if i < corners[1]{
                let pos = self.vertices[0].add(&unitvecs[0].scale((i as f64)*ds)).substract(&centro);
                vc.push(xyt{x: pos.x, y: pos.y, t: self.skin_thicknesses[0]})
            }
            else if i < corners[2]{
                let pos = self.vertices[1].add(&unitvecs[1].scale(((i-corners[1]) as f64)*ds)).substract(&centro);
                vc.push(xyt{x: pos.x, y: pos.y, t: self.skin_thicknesses[1]})
            }
            else if i < corners[3]{
                let pos = self.vertices[2].add(&unitvecs[2].scale(((i-corners[2]) as f64)*ds)).substract(&centro);
                vc.push(xyt{x: pos.x, y: pos.y, t: self.skin_thicknesses[2]})
            }
            else{
                let pos = self.vertices[3].add(&unitvecs[3].scale(((i-corners[3]) as f64)*ds)).substract(&centro);
                vc.push(xyt{x: pos.x, y: pos.y, t: self.skin_thicknesses[3]})
            }
        }

        //println!("{}, {}, {}, {}, {}", vc[corners[0]].x, vc[corners[1]].x, vc[corners[2]].x, vc[corners[3]].x, vc[corners[4]].x);
        //println!("{}, {}, {}, {}, {}", vc[corners[0]].y, vc[corners[1]].y, vc[corners[2]].y, vc[corners[3]].y, vc[corners[4]].y);

        (vc, corners, ds)

    }

    pub fn generateanalysable(&self) -> Analysable_section{
        let centroid = self.centroid();
        let inertias = self.inertia(centroid);
        let xytmapping = self.xytmap(centroid);

        Analysable_section{
            vertices: self.vertices,
            skin_thicknesses: self.skin_thicknesses,
            stringers: self.stringers,
            area_stringer: self.area_stringer,
            centroid: centroid,
            Ixx: inertias.0,
            Iyy: inertias.1,
            Ixy: inertias.2,
            xytmap: xytmapping.0,
            corners: xytmapping.1,
            ds: xytmapping.2,
        }
    }
}

pub struct Analysable_section{
    pub vertices: [Vec2; 4],
    pub skin_thicknesses: [f64; 4],
    pub stringers: [u16; 2],
    pub area_stringer: f64,
    pub centroid: Vec2,
    pub Ixx: f64,
    pub Iyy: f64,
    pub Ixy: f64,
    pub xytmap: Vec<xyt>,
    pub corners: [usize; 5],
    pub ds: f64,
}

impl Analysable_section{
    pub fn get_shear_flow_distribution(&self, torque: f64, shear: f64) -> Vec<f64>{
        let mut qs = self.get_qs0(shear);
        //println!("Qs 0: {}",qs.0);

        let Area = (self.vertices[0].substract(&self.vertices[3]).magnitude()+self.vertices[0].substract(&self.vertices[3]).magnitude())/2.*(self.vertices[0].x-self.vertices[1].x).abs();
        let q = torque/2./Area;
        for i in 0..self.xytmap.len(){
            qs.1[i] += qs.0+q;
        }

        qs.1
    }

    pub fn get_bending_stress_distribution(&self, moment: f64) -> Vec<f64>{
        let mut vc = Vec::new();

        let the_y_part = moment * self.Iyy;
        let the_x_part = -1. * moment * self.Ixy;
        let numerator = self.Ixx*self.Iyy-self.Ixy.powi(2);

        for p in &self.xytmap{
            vc.push((the_y_part*p.y + the_x_part*p.x)/numerator);
        }

        vc
    }

    pub fn get_max_stress(& self, torque: f64, shear: f64, moment: f64) -> ([f64; 4], [f64; 4]){
        let mut max_shear = self.get_shear_flow_distribution(torque, shear).max_per_section(&self.corners);
        for i in 0..4{
            max_shear[i] /= self.skin_thicknesses[i];
        }
        let max_stress = self.get_bending_stress_distribution(moment).max_per_section(&self.corners);

        (max_shear, max_stress)
    }

    fn qb(&self, s: usize, shear: f64) -> f64{
        let left_fraction = -1.*(shear*self.Iyy)/(self.Iyy*self.Ixx-self.Ixy.powi(2));
        let right_fraction = -1.*(-1.*shear*self.Ixy)/(self.Iyy*self.Ixx-self.Ixy.powi(2));

        let mut integral1 = 0.;
        let mut integral2 = 0.;

        for i in 0..s{
            integral1 += self.xytmap[i].y * self.xytmap[i].t * self.ds;
            integral2 += self.xytmap[i].x * self.xytmap[i].t * self.ds;
        }

        left_fraction*integral1+right_fraction*integral2
    }

    fn get_qs0(&self, shear: f64) -> (f64, Vec<f64>){
        let mut integral1 = 0.;
        let mut integral2 = 0.;
        let mut vc = Vec::new();

        for i in 0..self.xytmap.len(){
            let qb = self.qb(i, shear);
            integral1 += qb/self.xytmap[i].t*self.ds;
            integral2 += 1./self.xytmap[i].t*self.ds;
            vc.push(qb);
        }

        (-1. * integral1/integral2, vc)
    }

    pub fn get_weight_per_len(&self, rho: f64) -> f64{
        let mut length: [f64; 4] = [0.; 4];

        for i in 0..self.vertices.len(){
            length[i] = self.vertices[(i+1)%4].substract(&self.vertices[i]).magnitude();
        }

        let mut weights: Vec<f64> = Vec::new();

        for i in 0..length.len(){
            weights.push(length[i]*rho*self.skin_thicknesses[i]);
        }

        weights.push((self.stringers.iter().sum::<u16>() as f64) * self.area_stringer * rho);

        weights.into_iter().sum()
       
    }
}

pub trait Max_per_section{
    fn max_per_section(&self, corners: &[usize; 5]) -> [f64; 4];
}

impl Max_per_section for Vec<f64> {
    fn max_per_section(&self, corners: &[usize; 5]) -> [f64; 4]{
        let mut maxes: [f64; 4] = [0.; 4];
        for i in 0..self.len(){
            if i < corners[1]{
                if maxes[0].abs() < self[i].abs(){
                    maxes[0] = self[i];
                }
            }
            else if i < corners[2]{
                if maxes[1].abs() < self[i].abs(){
                    maxes[1] = self[i];
                }
            }
            else if i < corners[3]{
                if maxes[2].abs() < self[i].abs(){
                    maxes[2] = self[i];
                }
            }
            else{
                if maxes[3].abs() < self[i].abs(){
                    maxes[3] = self[i];
                }
            }
        }

        maxes
    }
}