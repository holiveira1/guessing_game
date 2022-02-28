//std é a biblioteca padrão
// io permite input de usuário e sua impressão
use std::io;
// usado para comparar valores
use std::cmp::Ordering;
// gerar números aleatórios
use rand::Rng;

// função main é o ponto de entrada dos programas
fn main() {
    // imprime mensagens
    println!("Guess the number!");

    //criando número aleatório
    // geramos um número entre 1 a 100
    // pode ser escrito também como
    // gen_range(1..=100)
    let secret_number = rand::thread_rng().
                            gen_range(1..101);

    // adiciona um loop infinito para o bloco de código
    loop {

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

        // convertemos a entrada de usuário para tipo numérico
        // ...: u32 equivale a um inteiro sem sinal de 32 bits
        // parse analisa uma string em tipo numérico
        // Usamos uma expressão match para testar se a entrada é válida
        let guess: u32 = match guess.trim().parse() {
            // parse retorna um tipo Result e um enum que possui Ok ou Err
            Ok(num) => num,
            // o _ indica que estamos falando de todos os erros possíveis
            Err(_) => continue,
        };

        // imprimindo o palpite do usuário
        println!("You guessed: {}", guess);

        // a expressão match decide o que fazer com uma comparação
        // cmp serve para comparar dois valores em qualquer coisa
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // pausa o loop se essa condição do match for atingida.
                break;
            }
        }

    }
}
