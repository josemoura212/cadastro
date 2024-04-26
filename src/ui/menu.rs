use crate::{logica::logica::*, models::aluno::{AcaoMenu, Aluno}};

use super::tela::{clear_screen, print_menu};



pub fn menu() {
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