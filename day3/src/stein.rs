const G: f64 = 9.81;

fn main() {
    let dt = 0.01;
    let mut v = 0.0;
    let mut s = 0.0;
    let mut t = 0.0;
    while s < 20.00 {
        v += G * dt;
        s+=v *dt;
        t += dt;
        
}
    
    
    println!("v nach 2 s: {:.3} m/s", v);
    println!("hier ist s{:.3} m", s);
    println!("Fallzeit: {:.3} s", t);
}
}