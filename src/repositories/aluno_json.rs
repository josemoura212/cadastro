use std::{fs::{self, File}, io::{Read, Write}, path::Path};

use crate::models::aluno::Aluno;


pub struct AlunoJsonRepo{
    pub path:String
}

impl AlunoJsonRepo{
    pub fn init(&self){
        let arquivo = File::open(&self.path);

        match arquivo {
            Ok(_) => (),
            Err(_) => {
                let path = Path::new(&self.path);
                if let Some(dir) = path.parent() {
                    fs::create_dir_all(dir).expect("Erro ao criar o diretório");
                }
                let _ = File::create(&self.path).expect("Erro ao criar o arquivo");
            }
        }
    }

    pub fn read(&self) -> Option<Vec<Aluno>>{
        let mut arquivo = File::open(&self.path).expect("Erro ao abrir o arquivo");
         
        let mut aluno_string = String::new();
        arquivo.read_to_string(&mut aluno_string).expect("Erro ao ler o arquivo");

        if aluno_string.is_empty(){
            return None;
        }

        let alunos:Vec<Aluno> = serde_json::from_str(&aluno_string).expect("Erro ao desserializar o arquivo");

        Some(alunos)
    }

    pub fn write(&self, aluno:Aluno){
        let alunos = self.read();

        match alunos {
            Some(_) => {
                let mut alunos = alunos.unwrap();
                alunos.push(aluno);
                let alunos_string = serde_json::to_string(&alunos).expect("Erro ao serializar o arquivo");

                let mut arquivo = File::create(&self.path).expect("Erro ao criar o arquivo");
                arquivo.write_all(alunos_string.as_bytes()).expect("Erro ao escrever no arquivo");
            },
            None => {
                let alunos = vec![aluno];

                let alunos_string = serde_json::to_string(&alunos).expect("Erro ao serializar o arquivo");

                let mut arquivo = File::create(&self.path).expect("Erro ao criar o arquivo");
                arquivo.write_all(alunos_string.as_bytes()).expect("Erro ao escrever no arquivo");
            }
        }
    }
}