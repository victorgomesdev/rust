fn main() {
    
    use std::io;

    let mut nome: String = String::new();

    println!("Digite seu nome:");

    io::stdin()
    .read_line(&mut nome)
    .expect("Erro");

    println!("Olá {}", nome);
}
