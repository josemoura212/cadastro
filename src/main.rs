use std::{fs::File, io::{Read, Write}};

use serde::{Deserialize, Serialize};

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

#[derive(Debug,Serialize,Deserialize)]
struct Aluno {
    nome: String,
    idade: i32,
    endereco: String,
}

fn main() -> std::io::Result<()>{
    println!("Iniciando sistema");

    let alunos =  vec![
        Aluno {
            nome: "João".to_string(),
            idade: 20,
            endereco: "Rua 1".to_string(),
        },
        Aluno {
            nome: "Maria".to_string(),
            idade: 22,
            endereco: "Rua 2".to_string(),
        }];

    let alunos_json = serde_json::to_string(&alunos)?;

    let mut arquivo = File::create("alunos.json")?;
    arquivo.write_all(alunos_json.as_bytes())?;
    
    let mut alunos_string = String::new();
    let _ = File::open("alunos.json")?.read_to_string(&mut alunos_string);

    let mut alunos: Vec<Aluno> = serde_json::from_str(&alunos_string)?;

    println!("{:?}", alunos);

    alunos.push(Aluno {
        nome: "José".to_string(),
        idade: 25,
        endereco: "Rua 3".to_string(),
    });

    let alunos_json = serde_json::to_string(&alunos)?;

    let mut arquivo = File::open("alunos.json")?;

    arquivo.write_all(alunos_json.as_bytes())?;
    
    arquivo.read_to_string(&mut alunos_string)?;

    let  alunos : Vec<Aluno> = serde_json::from_str(&alunos_string)?;

    println!("{:?}", alunos);

    // ui::menu::menu();

    Ok(())
}
