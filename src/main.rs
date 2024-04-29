use std::io;

use models::aluno::{Aluno, Nota};

//imports metodo 1 sem a criação de arquivos extras
mod models{
    pub mod aluno;
}
//imports metodo 2 sem o arquivo mod.rs
mod ui;
//imports metodo 3 com arquivo mod.rs
mod logica;
//metodo 4
#[path = "repositories/aluno_json.rs"]
mod aluno_json;

fn main() -> io::Result<()>{

    let aluno_repo = aluno_json::AlunoJsonRepo{path:"db/alunos.json".to_string()};

    aluno_repo.init();
    // ui::menu::menu();

    let aluno = Aluno{
        matricula: "1524".to_string(),
        nome: "José Augusto".to_string(),
        notas: vec![
            Nota{disciplina: "Matemática".to_string(), nota: 10.0},
            Nota{disciplina: "Português".to_string(), nota: 8.0},
        ]
    };

    aluno_repo.write(aluno);

    let alunos = aluno_repo.read().unwrap();

    for aluno in alunos{
        println!("{:?}", aluno);
    }

    Ok(())
}
