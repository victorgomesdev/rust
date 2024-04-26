
/*
Exemplo de módulo.

Um módulo pode ser declarado usando 'mod' ou 'pub mod';
Ele pode ser declarado dentro do mesmo arquivo em que é chamado:
Ex: src/main.rs

    mod teste{
        //stuff
    }

Pode ser dclarado em outro arquivo:
Ex: src/main.rs

    mod teste; // ele irá procurar por 'src/teste.rs' ou por 'src/teste/mod.rs'

As funções ou tipos dentro de um módulo podem ser feitas públicas através de 'pub';
Ex: src/teste.rs

    fn funcao1(){
        // Função privada, inacessível de fora do módulo
    }

    pub fn funcao2(){
        // Função acessível de fora do módulo
    }

É possível declarar submódulos usando as mesmas regras acima, porém o compilador irá procurar
pelos submódulos em um diretório com o nome do módulo pai ao invés do módulo filho.
Ex: src/teste.rs -> que é um módulo

    mod logs; // Irá procurar por 'src/testes/logs.rs' ou por 'src/testes/logs/mod.rs'
*/

pub fn teste(){// Função acessível atravéz de 'teste::teste()'
    println!("Olá");
}