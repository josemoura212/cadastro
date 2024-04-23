mod tela;
mod logica;
mod entidades;

use tela::*;
use entidades::Aluno;
use logica::*;

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
