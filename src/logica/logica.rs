use crate::{
    clear_screen, divider,
    models::aluno::{Aluno, Nota},
    timer,
};

pub fn register_grades(grades: &mut Vec<Nota>) {
    clear_screen();
    let mut discipline = String::new();
    println!("Digite a disciplina do aluno:");
    std::io::stdin()
        .read_line(&mut discipline)
        .expect("Erro ao ler a disciplina");

    discipline = discipline.trim().to_string();

    let mut grade = String::new();
    println!("\nDigite as notas do aluno:");
    std::io::stdin()
        .read_line(&mut grade)
        .expect("Erro ao ler as notas");

    let grade: f32 = match grade.trim().parse() {
        Ok(grade) => grade,
        Err(_) => {
            println!("Nota inválida");
            return register_grades(grades);
        }
    };

    let grade = Nota {
        disciplina: discipline,
        nota: grade,
    };

    grades.push(grade);

    clear_screen();

    let mut input = String::new();
    println!("Deseja adicionar mais notas? (s/n)");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a opção");

    if input.trim().to_lowercase() == "n" {
        return;
    }

    return register_grades(grades);
}

pub fn register_studant(studants: &mut Vec<Aluno>) {
    clear_screen();

    let mut registration = String::new();
    println!("\nDigite a matrícula do aluno:");
    std::io::stdin()
        .read_line(&mut registration)
        .expect("Erro ao ler a matrícula");

    registration = registration.trim().to_string();

    let registration = match buscar_aluno_por_matricula(&registration, studants) {
        Some(_) => {
            println!("Matrícula já cadastrada");
            let mut input = String::new();
            println!("Pressione enter para tentar novamente");
            std::io::stdin().read_line(&mut input).unwrap();
            return register_studant(studants);
        }
        None => registration,
    };

    let mut name = String::new();
    println!("Digite o nome do aluno:");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Erro ao ler o nome");

    name = name.trim().to_string();

    let mut grades: Vec<Nota> = Vec::new();

    register_grades(&mut grades);

    studants.push(Aluno {
        nome: name,
        matricula: registration,
        notas: grades,
    });
}

pub fn change_studant(studants: &mut Vec<Aluno>) {
    clear_screen();
    timer(1);
    if studants.is_empty() {
        println!("Não há alunos cadastrados");
        let mut input = String::new();
        println!("Pressione enter para continuar");
        std::io::stdin().read_line(&mut input).unwrap();
        return;
    }

    for aluno in studants.iter() {
        println!("Matricula: {} - Nome: {}", aluno.matricula, aluno.nome);
    }

    let mut matricula = String::new();
    println!(
        "Digite a matricula do aluno que deseja excluir. Ou pressione enter para voltar ao menu:"
    );
    std::io::stdin().read_line(&mut matricula).unwrap();

    if matricula.trim().is_empty() {
        return;
    }

    matricula = matricula.trim().to_string();

    let index = studants
        .iter()
        .position(|x| x.matricula == matricula)
        .unwrap();

    let studant = &mut studants[index];

    clear_screen();
    divider();

    println!("Nome atual: {}", studant.nome);
    println!("Deseja alterar o nome? (s/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() == "s" {
        let mut name = String::new();
        println!("Digite o novo nome:");
        std::io::stdin()
            .read_line(&mut name)
            .expect("Erro ao ler o nome");

        studant.nome = name.trim().to_string();
    }

    clear_screen();
    divider();

    println!("Matrícula atual: {}", studant.matricula);
    println!("Deseja alterar a matrícula? (s/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() == "s" {
        let mut registration = String::new();
        println!("Digite a nova matrícula:");
        std::io::stdin()
            .read_line(&mut registration)
            .expect("Erro ao ler a matrícula");

        studant.matricula = registration.trim().to_string();
    }

    clear_screen();
    divider();

    println!("Notas");
    for nota in &mut studant.notas {
        println!("Disciplina: {} - Nota: {}", &nota.disciplina, &nota.nota);
        println!("Deseja alterar a nota? (s/n)");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == "s" {
            let mut grade = String::new();
            println!("Digite a nova nota:");
            std::io::stdin()
                .read_line(&mut grade)
                .expect("Erro ao ler a nota");

            let grade: f32 = match grade.trim().parse() {
                Ok(grade) => grade,
                Err(_) => {
                    println!("Nota inválida");
                    return change_studant(studants);
                }
            };

            nota.nota = grade;
        }
    }
}

pub fn delete_studant(studants: &mut Vec<Aluno>) {
    clear_screen();
    timer(1);
    if studants.is_empty() {
        println!("Não há alunos cadastrados");
        let mut input = String::new();
        println!("Pressione enter para continuar");
        std::io::stdin().read_line(&mut input).unwrap();
        return;
    }

    for aluno in studants.iter() {
        println!("Matricula: {} - Nome: {}", aluno.matricula, aluno.nome);
    }

    let mut matricula = String::new();
    println!(
        "Digite a matricula do aluno que deseja excluir. Ou pressione enter para voltar ao menu:"
    );
    std::io::stdin().read_line(&mut matricula).unwrap();

    if matricula.trim().is_empty() {
        return;
    }

    matricula = matricula.trim().to_string();

    let aluno = buscar_aluno_por_matricula(&matricula, studants);

    let aluno = match aluno {
        Some(aluno) => aluno,
        None => {
            println!("Aluno não encontrado");
            let mut input = String::new();
            println!("Pressione enter para tentar novamente");
            std::io::stdin().read_line(&mut input).unwrap();
            return delete_studant(studants);
        }
    };

    let index = studants
        .iter()
        .position(|x| x.matricula == aluno.matricula)
        .unwrap();

    studants.remove(index);

    println!("Aluno excluído com sucesso");
    timer(2);
}

fn buscar_aluno_por_matricula<'a>(
    matricula: &'a String,
    studants: &'a Vec<Aluno>,
) -> Option<&'a Aluno> {
    for aluno in studants.iter() {
        if aluno.matricula == *matricula {
            return Some(aluno);
        }
    }
    return None;
}

pub fn list_studants(studants: &Vec<Aluno>) {
    clear_screen();
    timer(1);
    if studants.is_empty() {
        println!("Não há alunos cadastrados");
        let mut input = String::new();
        println!("Pressione enter para continuar");
        std::io::stdin().read_line(&mut input).unwrap();
        return;
    }
    println!("Alunos cadastrados:");
    for aluno in studants.iter() {
        divider();

        println!("Nome: {}", aluno.nome);
        println!("Matrícula: {}", aluno.matricula);
        println!("Notas:");
        for nota in &aluno.notas {
            println!("          {} : {}", nota.disciplina, nota.nota);
        }
        println!("Média: {:.2}", aluno.media());
        println!("Situação: {}", aluno.situacao());

        divider();
        let mut input = String::new();
        println!("Pressione enter para continuar");
        std::io::stdin().read_line(&mut input).unwrap();
        clear_screen();
    }
}
