fn f(x: f64) -> f64 {
    x * x
}

fn main() {
    let n = 100;
    let breite = 2.0 / n as f64;
    let mut flaeche = 0.0;

    for i in 0..n {
        let x = i as f64 * breite;
        flaeche = flaeche + f(x) * breite;
        println!("i{}, fläche = {:.4}", i, flaeche);
    }

    println!("over");
}