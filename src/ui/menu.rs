use crate::{
    aluno_json::{self, AlunoJsonRepo},
    logica::logica::*,
    models::aluno::AcaoMenu,
};

use super::tela::{clear_screen, print_menu};

pub fn menu() {
    let aluno_repo: AlunoJsonRepo = aluno_json::AlunoJsonRepo {
        path: "db/alunos.json".to_string(),
    };

    aluno_repo.init();

    loop {
        print_menu();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = match input.trim().parse::<AcaoMenu>() {
            Ok(acao) => acao,
            Err(_) => {
                println!("Opção inválida\n");
                continue;
            }
        };

        clear_screen();

        match input {
            AcaoMenu::CadastrarAluno => register_studant(&aluno_repo),
            AcaoMenu::AlterarAluno => change_studant(&aluno_repo),
            AcaoMenu::ExcluirAluno => delete_studant(&aluno_repo),
            AcaoMenu::ListarAlunos => list_studants(&aluno_repo),
            AcaoMenu::Sair => {
                println!("Saindo do sistema\n");
                break;
            }
        }
        clear_screen();
    }
}
