use triangle::{Triangle, Vec3, write_stl};

fn dihedral(x: f32) -> Vec3{
	Vec3{
		x: 0.,
		y: 0.,
		z: (3. as f32).to_radians().tan()*x,
	}
}

fn stringer(dx: Vec3, normal: Vec3, origin: &Vec3) -> Vec<Vec3>{
	let w = 0.05;
	let h = 0.05;
	let t = 0.01;
	let ortho = dx.cross(&normal).unitvec();
	let mut vc: Vec<Vec3> = Vec::new();
	vc.push(Vec3{
		x: 0.,
		y: 0.,
		z: 0.,
	}.add(origin));
	vc.push(ortho.scale(w).add(origin));
	vc.push(ortho.scale(w).add(&normal.scale(t)).add(origin));
	vc.push(ortho.scale(t).add(&normal.scale(t)).add(origin));
	vc.push(ortho.scale(t).add(&normal.scale(h)).add(origin));
	vc.push(ortho.scale(-1.*(w-t)).add(&normal.scale(h)).add(origin));
	vc.push(ortho.scale(-1.*(w-t)).add(&normal.scale(h-t)).add(origin));
	vc.push(ortho.scale(0.).add(&normal.scale(h-t)).add(origin));

	vc
}

fn stringer_along_line(line: Vec<Vec3>, normal: Vec<Vec3>) -> Vec<Triangle>{
	if line.len() != normal.len(){
		panic!("must have same len");
	}
	let mut sections: Vec<Vec<Vec3>> = Vec::new();
	//first create all the sections
	for i in 0..line.len(){
		if i != line.len()-1{
			sections.push(stringer(line[i+1].sub(&line[i]).unitvec(), normal[i].unitvec(), &line[i]));
		}
		else{
			sections.push(stringer(line[i].sub(&line[i-1]).unitvec(), normal[i].unitvec(), &line[i]));
		}
	}

	let mut triangles = Vec::new();

	for i in 0..sections.len()-1{
		//fucking tesselation
		triangles.push(Triangle::new(&sections[i][0], &sections[i+1][0], &sections[i+1][1]).invertn());
		triangles.push(Triangle::new(&sections[i][0], &sections[i][1], &sections[i+1][1]));

		triangles.push(Triangle::new(&sections[i][2], &sections[i][1], &sections[i+1][1]));
		triangles.push(Triangle::new(&sections[i][2], &sections[i+1][1], &sections[i+1][2]));

		triangles.push(Triangle::new(&sections[i][2], &sections[i][3], &sections[i+1][2]));
		triangles.push(Triangle::new(&sections[i][3], &sections[i+1][2], &sections[i+1][3]));

		triangles.push(Triangle::new(&sections[i][3], &sections[i][4], &sections[i+1][3]));
		triangles.push(Triangle::new(&sections[i][4], &sections[i+1][3], &sections[i+1][4]));

		triangles.push(Triangle::new(&sections[i][4], &sections[i][5], &sections[i+1][4]));
		triangles.push(Triangle::new(&sections[i][5], &sections[i+1][4], &sections[i+1][5]));

		triangles.push(Triangle::new(&sections[i][5], &sections[i][6], &sections[i+1][5]));
		triangles.push(Triangle::new(&sections[i][6], &sections[i+1][5], &sections[i+1][6]));

		triangles.push(Triangle::new(&sections[i][6], &sections[i][7], &sections[i+1][6]));
		triangles.push(Triangle::new(&sections[i][7], &sections[i+1][6], &sections[i+1][7]));

		triangles.push(Triangle::new(&sections[i][7], &sections[i][0], &sections[i+1][7]));
		triangles.push(Triangle::new(&sections[i][0], &sections[i+1][7], &sections[i+1][0]));
	}

	triangles
}

pub fn make_stl(sweep: f32, chord: &Fn(f32) -> f32, span: f32) -> Result<(), std::io::Error>{
    let fspar = 0.2;
    let rspar = 0.65;
    let vertices: [Vec3; 4] = [Vec3{x: 0., y: fspar, z: 0.},Vec3{x: 0., y: rspar, z: -0.0182},Vec3{x: 0., y: rspar, z: -0.1051},Vec3{x: 0., y: fspar, z: -0.1092}];
    let mut frontspar: Vec<Triangle> = Vec::new();
    let mut rearspar: Vec<Triangle> = Vec::new();
    let mut topskin: Vec<Triangle> = Vec::new();
	let mut botskin: Vec<Triangle> = Vec::new();

    //ugly incoming

	let upprsrf = [Vec3{x: 0., y:0.200000, z: -(0.054500+0.054700)},
	Vec3{x: 0., y:0.210000, z: -(0.055100+0.054700)},
	Vec3{x: 0., y:0.220000, z: -(0.055700+0.054700)},
	Vec3{x: 0., y:0.230000, z: -(0.056300+0.054700)},
	Vec3{x: 0., y:0.240000, z: -(0.056800+0.054700)},
	Vec3{x: 0., y:0.250000, z: -(0.057300+0.054700)},
	Vec3{x: 0., y:0.260000, z: -(0.057700+0.054700)},
	Vec3{x: 0., y:0.270000, z: -(0.058100+0.054700)},
	Vec3{x: 0., y:0.280000, z: -(0.058500+0.054700)},
	Vec3{x: 0., y:0.290000, z: -(0.058800+0.054700)},
	Vec3{x: 0., y:0.300000, z: -(0.059100+0.054700)},
	Vec3{x: 0., y:0.310000, z: -(0.059300+0.054700)},
	Vec3{x: 0., y:0.320000, z: -(0.059500+0.054700)},
	Vec3{x: 0., y:0.330000, z: -(0.059700+0.054700)},
	Vec3{x: 0., y:0.340000, z: -(0.059900+0.054700)},
	Vec3{x: 0., y:0.350000, z: -(0.060000+0.054700)},
	Vec3{x: 0., y:0.360000, z: -(0.060100+0.054700)},
	Vec3{x: 0., y:0.370000, z: -(0.060200+0.054700)},
	Vec3{x: 0., y:0.380000, z: -(0.060200+0.054700)},
	Vec3{x: 0., y:0.390000, z: -(0.060200+0.054700)},
	Vec3{x: 0., y:0.400000, z: -(0.060200+0.054700)},
	Vec3{x: 0., y:0.410000, z: -(0.060200+0.054700)},
	Vec3{x: 0., y:0.420000, z: -(0.060100+0.054700)},
	Vec3{x: 0., y:0.430000, z: -(0.060000+0.054700)},
	Vec3{x: 0., y:0.440000, z: -(0.059900+0.054700)},
	Vec3{x: 0., y:0.450000, z: -(0.059800+0.054700)},
	Vec3{x: 0., y:0.460000, z: -(0.059600+0.054700)},
	Vec3{x: 0., y:0.470000, z: -(0.059400+0.054700)},
	Vec3{x: 0., y:0.480000, z: -(0.059200+0.054700)},
	Vec3{x: 0., y:0.490000, z: -(0.058900+0.054700)},
	Vec3{x: 0., y:0.500000, z: -(0.058600+0.054700)},
	Vec3{x: 0., y:0.510000, z: -(0.058300+0.054700)},
	Vec3{x: 0., y:0.520000, z: -(0.058000+0.054700)},
	Vec3{x: 0., y:0.530000, z: -(0.057600+0.054700)},
	Vec3{x: 0., y:0.540000, z: -(0.057200+0.054700)},
	Vec3{x: 0., y:0.550000, z: -(0.056800+0.054700)},
	Vec3{x: 0., y:0.560000, z: -(0.056300+0.054700)},
	Vec3{x: 0., y:0.570000, z: -(0.055800+0.054700)},
	Vec3{x: 0., y:0.580000, z: -(0.055300+0.054700)},
	Vec3{x: 0., y:0.590000, z: -(0.054700+0.054700)},
	Vec3{x: 0., y:0.600000, z: -(0.054100+0.054700)},
	Vec3{x: 0., y:0.610000, z: -(0.053400+0.054700)},
	Vec3{x: 0., y:0.620000, z: -(0.052700+0.054700)},
	Vec3{x: 0., y:0.630000, z: -(0.052000+0.054700)},
	Vec3{x: 0., y:0.640000, z: -(0.051200+0.054700)},
	Vec3{x: 0., y:0.650000, z: -(0.050400+0.054700)}];
	
	let lowersrf = [Vec3{x: 0., y:0.200000, z: 0.054700-0.054700},
	Vec3{x: 0., y:0.210000, z: 0.055400-0.054700},
	Vec3{x: 0., y:0.220000, z: 0.056000-0.054700},
	Vec3{x: 0., y:0.230000, z: 0.056500-0.054700},
	Vec3{x: 0., y:0.240000, z: 0.057000-0.054700},
	Vec3{x: 0., y:0.250000, z: 0.057500-0.054700},
	Vec3{x: 0., y:0.260000, z: 0.057900-0.054700},
	Vec3{x: 0., y:0.270000, z: 0.058300-0.054700},
	Vec3{x: 0., y:0.280000, z: 0.058600-0.054700},
	Vec3{x: 0., y:0.290000, z: 0.058900-0.054700},
	Vec3{x: 0., y:0.300000, z: 0.059200-0.054700},
	Vec3{x: 0., y:0.310000, z: 0.059400-0.054700},
	Vec3{x: 0., y:0.320000, z: 0.059500-0.054700},
	Vec3{x: 0., y:0.330000, z: 0.059600-0.054700},
	Vec3{x: 0., y:0.340000, z: 0.059700-0.054700},
	Vec3{x: 0., y:0.350000, z: 0.059800-0.054700},
	Vec3{x: 0., y:0.360000, z: 0.059800-0.054700},
	Vec3{x: 0., y:0.370000, z: 0.059800-0.054700},
	Vec3{x: 0., y:0.380000, z: 0.059800-0.054700},
	Vec3{x: 0., y:0.390000, z: 0.059700-0.054700},
	Vec3{x: 0., y:0.400000, z: 0.059600-0.054700},
	Vec3{x: 0., y:0.410000, z: 0.059400-0.054700},
	Vec3{x: 0., y:0.420000, z: 0.059200-0.054700},
	Vec3{x: 0., y:0.430000, z: 0.058900-0.054700},
	Vec3{x: 0., y:0.440000, z: 0.058600-0.054700},
	Vec3{x: 0., y:0.450000, z: 0.058200-0.054700},
	Vec3{x: 0., y:0.460000, z: 0.057800-0.054700},
	Vec3{x: 0., y:0.470000, z: 0.057300-0.054700},
	Vec3{x: 0., y:0.480000, z: 0.056700-0.054700},
	Vec3{x: 0., y:0.490000, z: 0.056100-0.054700},
	Vec3{x: 0., y:0.500000, z: 0.055400-0.054700},
	Vec3{x: 0., y:0.510000, z: 0.054600-0.054700},
	Vec3{x: 0., y:0.520000, z: 0.053800-0.054700},
	Vec3{x: 0., y:0.530000, z: 0.052900-0.054700},
	Vec3{x: 0., y:0.540000, z: 0.051900-0.054700},
	Vec3{x: 0., y:0.550000, z: 0.050900-0.054700},
	Vec3{x: 0., y:0.560000, z: 0.049700-0.054700},
	Vec3{x: 0., y:0.570000, z: 0.048500-0.054700},
	Vec3{x: 0., y:0.580000, z: 0.047200-0.054700},
	Vec3{x: 0., y:0.590000, z: 0.045800-0.054700},
	Vec3{x: 0., y:0.600000, z: 0.044400-0.054700},
	Vec3{x: 0., y:0.610000, z: 0.042900-0.054700},
	Vec3{x: 0., y:0.620000, z: 0.041400-0.054700},
	Vec3{x: 0., y:0.630000, z: 0.039800-0.054700},
	Vec3{x: 0., y:0.640000, z: 0.038200-0.054700},
	Vec3{x: 0., y:0.650000, z: 0.034800-0.054700}];

    let mut x = 0.;
    let dx = 0.01;

	let mut ln = Vec::new();
	let mut nm = Vec::new();
	let mut ct = 0;

    while x+dx <= span{
		if ct%1 == 0{
			ln.push(Vec3{x: x, y: 0., z: 0.});
			nm.push(Vec3{x: 0., y: 0.5, z: -1.});
		}
		ct+=1;

        //frontspar
        let mut points: [Vec3; 4] = [Vec3{x:0.,y:0.,z:0.}; 4];
        //point 0 is at y and the lowest point
        points[0] = vertices[0].scale(chord(x)).add(&Vec3{x: x, y: x*sweep.tan(), z: 0.}).invertz().add(&dihedral(x));
        points[3] = vertices[3].scale(chord(x)).add(&Vec3{x: x, y: x*sweep.tan(), z: 0.}).invertz().add(&dihedral(x));

        points[1] = vertices[0].scale(chord(x+dx)).add(&Vec3{x: x+dx, y: (x+dx)*sweep.tan(), z: 0.}).invertz().add(&dihedral(x+dx));
        points[2] = vertices[3].scale(chord(x+dx)).add(&Vec3{x: x+dx, y: (x+dx)*sweep.tan(), z: 0.}).invertz().add(&dihedral(x+dx));

        frontspar.push(Triangle::new(&points[0],&points[1],&points[2]));
        frontspar.push(Triangle::new(&points[0],&points[2],&points[3]));

        //rearspar
        let mut points: [Vec3; 4] = [Vec3{x:0.,y:0.,z:0.}; 4];
        //point 0 is at y and the lowest point
        points[0] = vertices[1].scale(chord(x)).add(&Vec3{x: x, y: x*sweep.tan(), z: 0.}).invertz().add(&dihedral(x));
        points[3] = vertices[2].scale(chord(x)).add(&Vec3{x: x, y: x*sweep.tan(), z: 0.}).invertz().add(&dihedral(x));

        points[1] = vertices[1].scale(chord(x+dx)).add(&Vec3{x: x+dx, y: (x+dx)*sweep.tan(), z: 0.}).invertz().add(&dihedral(x+dx));
        points[2] = vertices[2].scale(chord(x+dx)).add(&Vec3{x: x+dx, y: (x+dx)*sweep.tan(), z: 0.}).invertz().add(&dihedral(x+dx));

        rearspar.push(Triangle::new(&points[0],&points[1],&points[2]));
        rearspar.push(Triangle::new(&points[0],&points[2],&points[3]));

        //skin
        for i in 0..upprsrf.len()-1{
            let mut points: [Vec3; 4] = [Vec3{x:0.,y:0.,z:0.}; 4];
            //point 0 is at y and the lowest point
            points[0] = upprsrf[i].scale(chord(x)).add(&Vec3{x: x, y: x*sweep.tan(), z: 0.}).invertz().add(&dihedral(x));
            points[1] = upprsrf[i+1].scale(chord(x)).add(&Vec3{x: x, y: x*sweep.tan(), z: 0.}).invertz().add(&dihedral(x));

            points[2] = upprsrf[i+1].scale(chord(x+dx)).add(&Vec3{x: x+dx, y: (x+dx)*sweep.tan(), z: 0.}).invertz().add(&dihedral(x+dx));
            points[3] = upprsrf[i].scale(chord(x+dx)).add(&Vec3{x: x+dx, y: (x+dx)*sweep.tan(), z: 0.}).invertz().add(&dihedral(x+dx));

            topskin.push(Triangle::new(&points[0],&points[1],&points[2]));
			topskin.push(Triangle::new(&points[0],&points[2],&points[3]));
        }

		for i in 0..lowersrf.len()-1{
            let mut points: [Vec3; 4] = [Vec3{x:0.,y:0.,z:0.}; 4];
            //point 0 is at y and the lowest point
            points[0] = lowersrf[i].scale(chord(x)).add(&Vec3{x: x, y: x*sweep.tan(), z: 0.}).invertz().add(&dihedral(x));
            points[1] = lowersrf[i+1].scale(chord(x)).add(&Vec3{x: x, y: x*sweep.tan(), z: 0.}).invertz().add(&dihedral(x));

            points[2] = lowersrf[i+1].scale(chord(x+dx)).add(&Vec3{x: x+dx, y: (x+dx)*sweep.tan(), z: 0.}).invertz().add(&dihedral(x+dx));
            points[3] = lowersrf[i].scale(chord(x+dx)).add(&Vec3{x: x+dx, y: (x+dx)*sweep.tan(), z: 0.}).invertz().add(&dihedral(x+dx));

            botskin.push(Triangle::new(&points[0],&points[1],&points[2]));
			botskin.push(Triangle::new(&points[0],&points[2],&points[3]));
        }

        x += dx;
    }

	if lowersrf.len() == upprsrf.len(){println!("nice");}

    /*write_stl("frontspar.stl", &frontspar)?;
    write_stl("rearspar.stl", &rearspar)?;
	write_stl("Topskin.stl", &topskin)?;
	write_stl("Botskin.stl", &botskin)?;*/

	let stringer = stringer_along_line(ln, nm);
	println!("{}", stringer.len());

	write_stl("teststr.stl", &stringer)?;

	/*let mut merged: Vec<Triangle> = Vec::new();
	merged.append(&mut frontspar);
	merged.append(&mut rearspar);
	merged.append(&mut topskin);
	merged.append(&mut botskin);

	write_stl("merged.stl", &merged)?;

    println!("saved");*/
    Ok(())
}