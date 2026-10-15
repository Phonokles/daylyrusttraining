fn main() {
    let mut s_vorher= 0.0;
    for i in 0..6{
        let t = i as f64;
        let s = 5.0 * t * t;
        let v = s-s_vorher;
        println!("t={} s, s ={} m, v={}", t, s, v);
        s_vorher = s;
    }
}
