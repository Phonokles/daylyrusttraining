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
    let b = Vektor { x: 4.0, y: 10.0 };
    let a = Vektor { x: 3.0, y: 4.0 };
    println!("Vektor a: {:?}", a);
    println!("Länge von a: {}", a.laenge());

    let c = a.plus(b);
    println!("a + b = {:?}", c);

    let d = a.mal(2.0);
    println!("a * 2 = {:?}", d);
    println!("Länge von a * 2: {}", d.laenge());
}