const G: f64 = 9.81;

#[derive(Debug, Clone, Copy)]
struct Vektor {
    x: f64,
    y: f64,
}

impl Vektor {
    fn laenge(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn plus(self, b: Vektor) -> Vektor {
        Vektor { x: self.x + b.x, y: self.y + b.y }
    }

    fn mal(self, m: f64) -> Vektor {
        Vektor { x: self.x * m, y: self.y * m }
    }
}

fn main() {
    let dt = 0.001;
    let a = Vektor { x: 0.0, y: -G };
    let mut v = Vektor { x: 10.0, y: 0.0 };
    let mut ort = Vektor { x: 0.0, y: 20.0 };
    let mut t = 0.0;

    while ort.y > 0.0 {
        v = v.plus(a.mal(dt));
        ort = ort.plus(v.mal(dt));
        t += dt;
    }

    println!("flugzeit: {:.2} s, weite: {:.2} m", t, ort.x);
    println!("aufprall tempo: {:.2} m/s", v.laenge());
}