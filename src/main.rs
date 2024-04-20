
fn main() {
    println!("Iniciando sistema \n");

    let options = vec![
    "1 - Cadastrar um novo aluno",
    "2 - Alterar aluno",
    "3 - Excluir aluno",
    "4 - Listar alunos cadastrados",
    "5 - Sair",
];
    loop {
        for i in &options {
            println!("{}", i);
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}
