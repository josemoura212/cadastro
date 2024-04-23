use std::{thread::sleep, time::Duration};

pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

pub fn timer(time: u64) {
    sleep(Duration::from_secs(time));
}

pub fn divider() {
    println!("{}", "-".repeat(40));
}

pub fn print_menu() {
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
