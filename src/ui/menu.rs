use crate::{
    aluno_json::AlunoJsonRepo,
    aluno_mysql::AlunoMySqlRepo,
    core_config::config,
    logica::{logica_aluno_json as json, logica_aluno_mysql as mysql},
    models::aluno::AcaoMenu,
};

use super::tela::{clear_screen, print_menu};

pub fn menu() {
    loop {
        println!("Bem vindo ao sistema de cadastro de alunos\n");
        println!("Escolha uma opção:\n");
        println!("1 - Banco Local");
        println!("2 - Banco MySQL");
        println!("3 - Sair");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("Banco Local Selecionado\n");
                clear_screen();
                loop_menu_local();
            }
            "2" => {
                println!("Banco MySQL Selecionado\n");
                clear_screen();
                loop_menu_mysql();
            }
            "3"=>{
                println!("Saindo");
                break;
            }
            _ => {
                println!("Opção inválida\n");
            }
        }
    }
}

fn loop_menu_local() {
    let aluno_repo: AlunoJsonRepo = AlunoJsonRepo {
        path: config::get_json_db_alunos_path(),
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
            AcaoMenu::CadastrarAluno => json::register_studant(&aluno_repo),
            AcaoMenu::AlterarAluno => json::change_studant(&aluno_repo),
            AcaoMenu::ExcluirAluno => json::delete_studant(&aluno_repo),
            AcaoMenu::ListarAlunos => json::list_studants(&aluno_repo),
            AcaoMenu::Sair => {
                println!("Saindo do sistema\n");
                break;
            }
        }
        clear_screen();
    }
}

fn loop_menu_mysql() {
    let aluno_repo = AlunoMySqlRepo::new(&config::get_mysql_db_alunos_path());

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
            AcaoMenu::CadastrarAluno => mysql::register_studant(&aluno_repo),
            AcaoMenu::AlterarAluno => mysql::change_studant(&aluno_repo),
            AcaoMenu::ExcluirAluno => mysql::delete_studant(&aluno_repo),
            AcaoMenu::ListarAlunos => mysql::list_studants(&aluno_repo),
            AcaoMenu::Sair => {
                println!("Saindo do sistema\n");
                break;
            }
        }
        clear_screen();
    }
}
