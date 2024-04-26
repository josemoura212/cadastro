//imports metodo 1 sem a criação de arquivos extras
mod models{
    pub mod aluno;
}
//imports metodo 2 sem o arquivo mod.rs
mod ui;
//imports metodo 3 com arquivo mod.rs
mod logica;

use models::aluno::Aluno;
use ui::tela::*;
use logica::logica::*;

use crate::models::aluno::AcaoMenu;


fn main() {
    println!("Iniciando sistema");

    let mut studants: Vec<Aluno> = Vec::new();

    loop {
        print_menu();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = match input.trim().parse::<AcaoMenu>(){
            Ok(acao) => acao,
            Err(_) => {
                println!("Opção inválida\n");
                continue;
            }
        };

        clear_screen();

        match input {
            AcaoMenu::CadastrarAluno => register_studant(&mut studants),
            AcaoMenu::AlterarAluno => change_studant(&mut studants),
            AcaoMenu::ExcluirAluno => delete_studant(&mut studants),
            AcaoMenu::ListarAlunos => list_studants(&studants),
            AcaoMenu::Sair => {
                println!("Saindo do sistema\n");
                break;
            }
        }
        clear_screen();
    }
}
