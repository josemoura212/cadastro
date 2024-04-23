use std::str::FromStr;

pub struct Aluno {
    pub nome: String,
    pub matricula: String,
    pub notas: Vec<Nota>,
}
pub struct Nota {
    pub disciplina: String,
    pub nota: f32,
}

#[derive(Debug)]
pub enum AcaoMenu{
    CadastrarAluno,
    AlterarAluno,
    ExcluirAluno,
    ListarAlunos,
    Sair,
}

impl FromStr for AcaoMenu{
    type Err = ();

    fn from_str(s: &str) -> Result<Self,Self::Err>{
        match s.to_lowercase().trim(){
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
