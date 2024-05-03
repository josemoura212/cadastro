use mysql::{params, prelude::Queryable, Pool, PooledConn, TxOpts};

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

    pub fn get_all(&self) -> Option<Vec<Aluno>> {
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
                        .split(';')
                        .map(|nota| {
                            let mut nota = nota.split(':');
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

    pub fn save(&self, aluno: Aluno) {
        let mut conn = self.get_conn();
        let mut tx = conn.start_transaction(TxOpts::default()).unwrap();

        tx.exec_drop(
            "INSERT INTO aluno (matricula, nome) VALUES (:matricula, :nome)",
            params! {
                "matricula" => &aluno.matricula,
                "nome" => aluno.nome,
            },
        )
        .unwrap();

        for nota in &aluno.notas {
            tx.exec_drop(
                "INSERT INTO nota (matricula, materia, nota) VALUES (:matricula, :materia, :nota)",
                params! {
                    "matricula" => &aluno.matricula,
                    "materia" => &nota.disciplina,
                    "nota" => &nota.nota,
                },
            )
            .unwrap();
        }

        tx.commit().unwrap();
    }

    pub fn change(&self, aluno: Aluno) {
        let mut conn = self.get_conn();

        conn.exec_drop(
            "UPDATE aluno SET nome = :nome WHERE matricula = :matricula",
            params!(
                "nome" => aluno.nome,
                "matricula" => &aluno.matricula
            ),
        )
        .unwrap();

        for nota in &aluno.notas {
            conn.exec_drop(
                "UPDATE nota SET nota = :nota WHERE matricula = :matricula AND materia = :materia",
                params!(
                    "nota" => &nota.nota,
                    "matricula" => &aluno.matricula,
                    "materia" => &nota.disciplina
                ),
            )
            .unwrap();
        }
    }

    pub fn delete(&self, matricula: String) {
        let mut conn = self.get_conn();
        let mut tx = conn.start_transaction(TxOpts::default()).unwrap();

        tx.exec_drop(
            "DELETE FROM nota WHERE matricula = :matricula",
            params!(
                "matricula" => &matricula
            ),
        )
        .unwrap();

        tx.exec_drop(
            "DELETE FROM aluno WHERE matricula = :matricula",
            params!(
                "matricula" => &matricula
            ),
        )
        .unwrap();

        tx.commit().unwrap();
    }
}
