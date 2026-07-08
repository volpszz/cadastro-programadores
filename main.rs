use std::io;
struct Programador {
    nome: String,
    idade: u8,
    linguagem: Vec<String>,
}

fn main() {
    let mut usuario_nome = String::new();
    println!("Digite seu nome:");
    io::stdin()
        .read_line(&mut usuario_nome)
        .expect("falha ao ler!");

    println!("Digite sua idade:");
    let mut usuario_input = String::new();
    io::stdin()
        .read_line(&mut usuario_input)
        .expect("falha ao ler!");
    let usuario_idade: u8 = usuario_input
        .trim()
        .parse()
        .expect("digite um numero valido: ");

    println!("Digite suas linguagens ('Fim' pra terminar):");
    let mut usuario_linguagem: Vec<String> = Vec::new();
    loop {
        let mut usuario_input = String::new();
        io::stdin()
            .read_line(&mut usuario_input)
            .expect("digite algo valido!");
        let usuario_input_limpo = usuario_input.trim().to_lowercase();
        if (usuario_input_limpo) == "fim" || usuario_input_limpo.is_empty() {
            break;
        } else {
            usuario_linguagem.push(usuario_input_limpo.to_string());
            println!("Adicionando!")
        }
    }
    let dev = Programador {
        nome: usuario_nome.trim().to_string(),
        idade: usuario_idade,
        linguagem: usuario_linguagem,
    };

    println!("\n =========PROGRAMADOR CADASTRADO======");
    println!("nome: {}", dev.nome);
    println!("idade: {}", dev.idade);
    println!("linguagens: {:?}", dev.linguagem);
    println!("=============PRESSIONE QUALQUER TECLA PRA SAIR============");
    let mut usuario_sair = String::new();
    io::stdin()
        .read_line(&mut usuario_sair)
        .expect("falha ao ler!");
    match usuario_sair {
        _ => println!("....."),
    }
}
