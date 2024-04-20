use std::thread::sleep;
use std::time::Duration;

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
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
        "1 - Cadastrar um novo aluno",
        "2 - Alterar aluno",
        "3 - Excluir aluno",
        "4 - Listar alunos cadastrados",
        "5 - Sair",
    ];
    divider();
    for i in &options {
        println!("{}", i);
    }
    divider();
}

fn list_studants(studants: &Vec<(String, String, Vec<(String, f32)>)>) {
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
    for i in studants.iter() {
        divider();
        println!("Nome: {}", i.0);
        println!("Matrícula: {}", i.1);
        println!("Notas:");
        for j in i.2.iter() {
            println!("      {} : {}", j.0,j.1);
        }
        divider();
        let mut input = String::new();
        println!("Pressione enter para continuar");
        std::io::stdin().read_line(&mut input).unwrap();
        clear_screen();
    }
}

fn register_grades(grades: &mut Vec<(String, f32)>) {
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

    grades.push((discipline, grade));

    clear_screen();

    let mut input = String::new();
    println!("Deseja adicionar mais notas? (s/n)");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a opção");

    if input.trim() == "n" {
        return;
    }

    return register_grades(grades);
}

fn register_studant(studants: &mut Vec<(String, String, Vec<(String, f32)>)>) {
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

    let mut grades: Vec<(String, f32)> = Vec::new();

    register_grades(&mut grades);

    let studant = (name, registration, grades);
    studants.push(studant);
}

fn main() {
    println!("Iniciando sistema");

    let mut studants: Vec<(String, String, Vec<(String, f32)>)> = Vec::new();

    loop {
        print_menu();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        clear_screen();

        match input.trim() {
            "1" => register_studant(&mut studants),
            "2" => println!("Iniciando alteração de aluno\n"),
            "3" => println!("Iniciando exclusão de aluno\n"),
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
