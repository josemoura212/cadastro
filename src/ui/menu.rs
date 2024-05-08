use crate::{
    aluno_json::AlunoJsonRepo,
    aluno_mysql::AlunoMySqlRepo,
    core_config::config,
    logica::{logica_aluno_json as json, logica_aluno_mysql as mysql},
    models::aluno::AcaoMenu,
};

use super::tela::{clear_screen, print_menu};

#[derive(Debug, PartialEq, PartialOrd)]
enum DBType {
    Local,
    MySQL,
}

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
                lopp_menu(DBType::Local)
            }
            "2" => {
                println!("Banco MySQL Selecionado\n");
                clear_screen();
                lopp_menu(DBType::MySQL)
            }
            "3" => {
                println!("Saindo");
                break;
            }
            _ => {
                println!("Opção inválida\n");
            }
        }
    }
}

fn lopp_menu(db: DBType) {
    let aluno_repo_json = AlunoJsonRepo {
        path: config::get_json_db_alunos_path(),
    };
    let aluno_repo_mysql = AlunoMySqlRepo::new(&config::get_mysql_db_alunos_path());
    if db == DBType::Local {
        aluno_repo_json.init();
    }

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

        if db == DBType::Local {
            match input {
                AcaoMenu::CadastrarAluno => json::register_studant(&aluno_repo_json),
                AcaoMenu::AlterarAluno => json::change_studant(&aluno_repo_json),
                AcaoMenu::ExcluirAluno => json::delete_studant(&aluno_repo_json),
                AcaoMenu::ListarAlunos => json::list_studants(&aluno_repo_json),
                AcaoMenu::Sair => {
                    println!("Saindo do sistema\n");
                    break;
                }
            }
        } else {
            match input {
                AcaoMenu::CadastrarAluno => mysql::register_studant(&aluno_repo_mysql),
                AcaoMenu::AlterarAluno => mysql::change_studant(&aluno_repo_mysql),
                AcaoMenu::ExcluirAluno => mysql::delete_studant(&aluno_repo_mysql),
                AcaoMenu::ListarAlunos => mysql::list_studants(&aluno_repo_mysql),
                AcaoMenu::Sair => {
                    println!("Saindo do sistema\n");
                    break;
                }
            }
        }

        clear_screen();
    }
}
