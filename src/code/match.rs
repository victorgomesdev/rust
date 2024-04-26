//Exemplo de uso do match - control flow

enum Moedas{
    Real,
    Dolar,
    Euro,
    Libra
}

fn descobrir_moeda(moeda: Moedas)-> String{

    match moeda {
        Moedas::Dolar => {
            String::from("Dólar: USD")
        },
        Moedas::Euro =>{
            String::from("Euro: EUR")
        },
        Moedas::Libra =>{
            String::from("Libra: GDP")
        },
        Moedas::Real =>{
            String::from("Real: BRL")
        }
    }
}

println!("{}", descobrir_moeda(Moedas::Euro).to_uppercase());
println!("{}", descobrir_moeda(Moedas::Dolar).to_uppercase());
println!("{}", descobrir_moeda(Moedas::Libra).to_uppercase());
println!("{}", descobrir_moeda(Moedas::Real).to_uppercase());