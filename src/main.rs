mod entidades;
mod logica;
mod tela;

use entidades::Aluno;
use logica::*;
use tela::*;

use crate::entidades::AcaoMenu;

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
