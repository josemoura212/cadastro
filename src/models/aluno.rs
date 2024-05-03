use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aluno {
    pub nome: String,
    pub matricula: String,
    pub notas: Vec<Nota>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nota {
    pub disciplina: String,
    pub nota: f32,
}

impl Aluno {
    pub fn media(&self) -> f32 {
        let mut soma = 0.0;
        for nota in &self.notas {
            soma += nota.nota;
        }
        soma / self.notas.len() as f32
    }

    pub fn situacao(&self) -> String {
        let media = self.media();
        if media >= 7.0 {
            return "Aprovado".to_string();
        } else if media >= 5.0 {
            return "Recuperação".to_string();
        }

        "Reprovado".to_string()
    }
}

#[derive(Debug)]
pub enum AcaoMenu {
    CadastrarAluno,
    AlterarAluno,
    ExcluirAluno,
    ListarAlunos,
    Sair,
}

impl FromStr for AcaoMenu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "1" => Ok(AcaoMenu::CadastrarAluno),
            "cadastrar" => Ok(AcaoMenu::CadastrarAluno),
            "2" => Ok(AcaoMenu::AlterarAluno),
            "alterar" => Ok(AcaoMenu::AlterarAluno),
            "3" => Ok(AcaoMenu::ExcluirAluno),
            "excluir" => Ok(AcaoMenu::ExcluirAluno),
            "4" => Ok(AcaoMenu::ListarAlunos),
            "listar" => Ok(AcaoMenu::ListarAlunos),
            "5" => Ok(AcaoMenu::Sair),
            _ => Err(()),
        }
    }
}
