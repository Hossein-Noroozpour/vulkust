
#[derive(Clone)]
struct Mat {
	e: Vec<Vec<String>>,
}

impl Mat {
	pub fn new(ec: usize) -> Self {
		let mut e = Vec::new();
		for i in 0..ec {
			let mut r = Vec::new();
			for j in 0..ec {
				r.push(format!("self.data[{}][{}]", i, j));
			}
			e.push(r);
		}
		Mat {
			e: e,
		}
	}

	fn det2(&self, ident: usize) -> String {
		format!("({} * {}) - ({} * {})", self.e[0][0], self.e[1][1], self.e[0][1], self.e[1][0])
	}

	pub fn det(&self, ident: usize) -> String {
		if self.e.len() == 2 {
			return self.det2(ident + 1);
		}

		let mut ms = Vec::new();
		for i in 0..self.e.len() {
			let mut m = Mat {
				e: Vec::new(),
			};
			for j in 1..self.e.len() {
				let mut r = Vec::new();
				for k in 0..self.e.len() {
					if k != i {
						r.push(self.e[j][k].clone());
					}
				}
				m.e.push(r);
			}
			ms.push(m);
		}

		let mut s = String::new();

		for i in 0..self.e.len() {
			s = format!("{}\n{} {} ({} * ({}))", 
				s, 
				"\t".repeat(ident),
				if (i & 1) == 1 {
					"-"
				} else {
					"+"
				},
				self.e[0][i], ms[i].det(ident + 1)
			);
		}
		return s;
	}

	pub fn minor(&self, ri: usize, ci: usize) -> Mat {
		let mut m = Mat {
			e: Vec::new(),
		};
		for i in 0..self.e.len() {
			if i != ri {
				let mut r = Vec::new();	
				for j in 0..self.e.len() {
					if j != ci {
						r.push(self.e[i][j].clone());
					}
				}
				m.e.push(r);
			}
		}
		return m;
	}

	pub fn inverse(&self) -> String {
		let mut s = format!("let d = self.det();\n");
		s = format!("{}let inv = Matx {{\n\tdata: [\n", s);
		for i in 0..self.e.len() {
			s = format!("{}\t\t[\n", s);
			for j in 0..self.e.len() {
				s = format!("{}\t\t\t({}\n\t\t\t) / d,\n", s, self.minor(i, j).det(4));
			}
			s = format!("{}\t\t],\n", s);
		}
		return s;
	}
}

fn main() {
	let m = Mat {
		e: vec![
			vec!["a".to_string(), "b".to_string(), "c".to_string()],
			vec!["d".to_string(), "e".to_string(), "f".to_string()],
			vec!["g".to_string(), "h".to_string(), "i".to_string()],
		],
	};
    println!("det {}", m.det(0));
	let m = Mat::new(3);
    println!("det {}", m.det(0));
    println!("inverse:\n{}", m.inverse());
	let m = Mat::new(4);
    println!("det {}", m.det(0));
    println!("inverse:\n{}", m.inverse());
}
