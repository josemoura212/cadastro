use std::thread::sleep;
use std::time::Duration;

fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

fn timer(time: u64) {
    sleep(Duration::from_secs(time));
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
    println!("---------------------------------");
    for i in &options {
        println!("{}", i);
    }
    println!("---------------------------------");
}

fn list_studants(studants: &Vec<(String,String,Vec<(String,f32)>)>) {
    println!(
        "---------------------------------\nListando alunos\n---------------------------------"
    );
    timer(1);
    println!("Alunos cadastrados:");
    for i in studants.iter() {
        println!("Nome: {}", i.0);
        println!("Matrícula: {}", i.1);
        println!("Notas:");
        for j in i.2.iter() {
            println!("      {} : {}", j.0,j.1);
        }
        println!("---------------------------------");
    }

    timer(10);
}

fn register_studant(studants: &mut Vec<(String,String,Vec<(String,f32)>)>) {
    println!("---------------------------------\nIniciando cadastro de aluno\n---------------------------------");
    timer(1);

    let mut name = String::new();
    println!("\nDigite o nome do aluno:");
    std::io::stdin().read_line(&mut name).expect("Erro ao ler o nome");

    let mut registration = String::new();
    println!("\nDigite a matrícula do aluno:");
    std::io::stdin().read_line(&mut registration).expect("Erro ao ler a matrícula");

    let mut grades: Vec<(String,f32)> = Vec::new();

    loop {
        let mut discipline = String::new();
        println!("\nDigite a disciplina do aluno:");
        std::io::stdin().read_line(&mut discipline).expect("Erro ao ler a disciplina");

        let mut grade = String::new();
        println!("\nDigite as notas do aluno:");
        std::io::stdin().read_line(&mut grade).expect("Erro ao ler as notas");

        let grade: f32 = match grade.trim().parse() {
            Ok(grade) => grade,
            Err(_) => {
                println!("Nota inválida");
                continue;
            }
        };

        grades.push((discipline.trim().to_string(), grade));

        clear_screen();

        let mut input = String::new();
        println!("Deseja adicionar mais notas? (s/n)");
        std::io::stdin().read_line(&mut input).expect("Erro ao ler a opção");

        if input.trim() == "n" {
            break;
        }
    }

    let studant = (name.trim().to_string(), registration.trim().to_string(), grades);
    studants.push(studant);
}

fn main() {
    println!("Iniciando sistema");

    let mut studants: Vec<(String,String,Vec<(String,f32)>)> = Vec::new();

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
