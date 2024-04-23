use std::thread::sleep;
use std::time::Duration;

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

struct Aluno {
    nome: String,
    matricula: String,
    notas: Vec<Nota>,
}

struct Nota {
    disciplina: String,
    nota: f32,
}

fn timer(time: u64) {
    sleep(Duration::from_secs(time));
}

fn divider() {
    println!("{}", "-".repeat(40));
}

fn print_menu() {
    let options = vec![
        "Escolha uma opção:",
        "1 - Cadastrar aluno",
        "2 - Alterar aluno",
        "3 - Excluir aluno",
        "4 - Listar alunos",
        "5 - Sair",
    ];
    divider();
    for i in &options {
        println!("{}", i);
    }
    divider();
}

fn register_grades(grades: &mut Vec<Nota>) {
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

fn register_studant(studants: &mut Vec<Aluno>) {
    clear_screen();

    let mut name = String::new();
    println!("Digite o nome do aluno:");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Erro ao ler o nome");

    name = name.trim().to_string();

    let mut registration = String::new();
    println!("\nDigite a matrícula do aluno:");
    std::io::stdin()
        .read_line(&mut registration)
        .expect("Erro ao ler a matrícula");

    registration = registration.trim().to_string();

    let mut grades: Vec<Nota> = Vec::new();

    register_grades(&mut grades);

    studants.push(Aluno {
        nome: name,
        matricula: registration,
        notas: grades,
    });
}

fn change_studant(studants: &mut Vec<Aluno>) {
    clear_screen();
    timer(1);
    if studants.is_empty() {
        println!("Não há alunos cadastrados");
        let mut input = String::new();
        println!("Pressione enter para continuar");
        std::io::stdin().read_line(&mut input).unwrap();
        return;
    }

    for (index, aluno) in studants.iter().enumerate() {
        println!("id: {} - Nome: {}", index, aluno.nome);
    }

    let mut index = String::new();
    println!("Digite o id do aluno que deseja excluir. Ou pressione enter para voltar ao menu:");
    std::io::stdin().read_line(&mut index).unwrap();

    let index: usize = match index.trim().parse() {
        Ok(index) => index,
        Err(_) => {
            return;
        }
    };

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

fn delete_studant(studants: &mut Vec<Aluno>) {
    clear_screen();
    timer(1);
    if studants.is_empty() {
        println!("Não há alunos cadastrados");
        let mut input = String::new();
        println!("Pressione enter para continuar");
        std::io::stdin().read_line(&mut input).unwrap();
        return;
    }

    for  aluno in studants.iter() {
        println!("matricula: {} - Nome: {}", aluno.matricula, aluno.nome);
    }

    let mut matricula = String::new();
    println!("Digite a matricula do aluno que deseja excluir. Ou pressione enter para voltar ao menu:");
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

    let index = studants.iter().position(|x| x.matricula == aluno.matricula).unwrap();

    studants.remove(index);

    println!("Aluno excluído com sucesso");
    timer(2);

}

fn buscar_aluno_por_matricula<'a>(matricula: &'a String, studants: &'a Vec<Aluno>) -> Option<&'a Aluno> {
    for aluno in studants.iter() {
        if aluno.matricula == *matricula {
            return Some(aluno);
        }
    }
    return None;
}

fn list_studants(studants: &Vec<Aluno>) {
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
        divider();
        let mut input = String::new();
        println!("Pressione enter para continuar");
        std::io::stdin().read_line(&mut input).unwrap();
        clear_screen();
    }
}

fn main() {
    println!("Iniciando sistema");

    let mut studants: Vec<Aluno> = Vec::new();

    loop {
        print_menu();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        clear_screen();

        match input.trim() {
            "1" => register_studant(&mut studants),
            "2" => change_studant(&mut studants),
            "3" => delete_studant(&mut studants),
            "4" => list_studants(&studants),
            "5" => {
                println!("Saindo do sistema\n");
                break;
            }
            _ => println!("Opção inválida\n"),
        }
        clear_screen();
    }
}
