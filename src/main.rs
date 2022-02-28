//std é a biblioteca padrão
// io permite input de usuário e sua impressão
use std::io;

// função main é o ponto de entrada dos programas
fn main() {
    // imprime mensagens
    println!("Guess the number!");

    println!("Please input your guess.");

    // let é usado para definir variáveis, que por padrão são imutáveis
    // mut deixa as variáveis mutáveis
    // instanciamos a variável guess com uma String
    let mut guess = String::new();

    // stdin permite manipular entradas de usuários
    io::stdin()
        // Faz a leitura da entrada o usuário e coloca na variável guess
        // O &indica que esse argumento é uma referência ,
        // que fornece uma maneira de permitir que várias partes do seu código acessem um dado
        // sem precisar copiar esses dados na memória várias vezes.
        // assim como as variáveis, as referências são imutáveis por padrão
        // colocamos o mut para deixar mutável
        .read_line(&mut guess)
        // define a mensagem de retorno caso aconteça falhas
        .expect("Failed to read line");

        // imprimindo o palpite do usuário
        println!("You guessed: {}", guess);

}
