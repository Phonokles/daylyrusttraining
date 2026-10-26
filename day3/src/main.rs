const G: f64 = 9.81;

fn main() {
    let dt = 0.01;
    let schritte = (2.0 / dt) as i32;
    let mut v = 0.0;
    let mut s = 0.0;
    for _ in 0..schritte{
        v += G * dt;
        s+=v *dt;
    }
    
    println!("v nach 2 s: {:.3} m/s", v);
    println!("hier ist s{:.3} m", s);
}