use std::io;

fn soma(x: &String, y: &String) -> f64 {
    let x = x.trim().parse::<f64>().unwrap();
    let y = y.trim().parse::<f64>().unwrap();
    x + y
}
fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Digite o valor de A: ");
    io::stdin().read_line(&mut a).expect("Error reading");
    println!("Digite o valor de B: ");
    io::stdin().read_line(&mut b).expect("Error reading");
    println!("Digite o valor de C: ");
    io::stdin().read_line(&mut c).expect("Error reading");

    let mut x = soma(&A, &B);
    println!("A soma do valor de A e B é: {x}");
    x = soma(&A, &C);
    println!("A soma do valor de A e B é: {x}");
    x = soma(&B, &C);
    println!("A soma do valor de A e B é: {x}");
}
