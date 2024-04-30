use mysql::{prelude::Queryable, Pool, PooledConn};

use crate::models::aluno::{Aluno, Nota};

pub struct AlunoMySqlRepo {
    pool: Pool,
}

impl AlunoMySqlRepo {
    pub fn new(connection: &str) -> Self {
        AlunoMySqlRepo {
            pool: Pool::new(connection).unwrap(),
        }
    }

    fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }

    pub fn read(&self) -> Option<Vec<Aluno>> {
        let mut conn = self.get_conn();

        let query = "SELECT 
                                alu.matricula,
                                alu.nome,
                            group_concat(nota.materia,':',nota.nota separator ';') as notas 
                            FROM aluno alu inner join nota on alu.matricula = nota.matricula
                            GROUP BY alu.matricula;";
        let alunos: Vec<Aluno> = conn
            .query_map(
                query,
                |(matricula, nome, notas): (String, String, String)| Aluno {
                    matricula,
                    nome,
                    notas: notas
                        .split(";")
                        .map(|nota| {
                            let mut nota = nota.split(":");
                            Nota {
                                disciplina: nota.next().unwrap().to_string(),
                                nota: nota.next().unwrap().parse().unwrap(),
                            }
                        })
                        .collect(),
                },
            )
            .unwrap();

        Some(alunos)
    }

    // pub fn save(&self, aluno: Aluno) {
    //     let mut query = "INSERT INTO aluno (matricula, nome) VALUES (?, ?)".to_string();
    //     let mut params = vec![&aluno.matricula, &aluno.nome];

    //     for nota in &aluno.notas {
    //         query = format!("{}; INSERT INTO nota (matricula, materia, nota) VALUES (?, ?, ?)", query);
    //         params.push(&nota.disciplina);
    //         params.push(&nota.nota.to_string());
    //     }

    //     let mut conn = self.get_conn();
    //     conn.exec_drop(query, params).unwrap();
    // }

    // pub fn alter(&self, aluno: Aluno) {
    //     let mut query = "UPDATE aluno SET nome = ? WHERE matricula = ?".to_string();
    //     let mut params = vec![&aluno.nome, &aluno.matricula];

    //     for nota in &aluno.notas {
    //         query = format!("{}; UPDATE nota SET nota = ? WHERE matricula = ? AND materia = ?", query);
    //         params.push(&nota.nota.to_string());
    //         params.push(&aluno.matricula);
    //         params.push(&nota.disciplina);
    //     }

    //     let mut conn = self.get_conn();
    //     conn.exec_drop(query, params).unwrap();
    // }

    // pub fn delete(&self, matricula: String) {
    //     let query = "DELETE FROM aluno WHERE matricula = ?; DELETE FROM nota WHERE matricula = ?"
    //         .to_string();
    //     let params = vec![&matricula, &matricula];

    //     let mut conn = self.get_conn();
    //     conn.exec_drop(query, params).unwrap();
    // }

    // pub fn save(&self,aluno:Aluno){
    //     let mut query = "INSERT INTO aluno (matricula,nome) VALUES(?,?)".to_string();

    //     for nota in aluno.notas{
    //         query = format!("{};INSERT INTO nota (matricula,nota) VALUES(?,?)",query);
    //     }
    // }

    // pub fn alter(&self,aluno:Aluno){
    //     let mut query = "UPDATE aluno SET nome = ? WHERE matricula = ?".to_string();

    //     for nota in aluno.notas{
    //         query = format!("{};UPDATE nota SET nota = ? WHERE matricula = ?",query);
    //     }
    // }

    // pub fn delete(&self,matricula:String){
    //     let query = "DELETE FROM aluno WHERE matricula = ?;delete from nota where matricula = ?;".to_string();
    // }
}
