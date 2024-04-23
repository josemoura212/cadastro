pub struct Aluno {
    pub nome: String,
    pub matricula: String,
    pub notas: Vec<Nota>,
}
pub struct Nota {
    pub disciplina: String,
    pub nota: f32,
}