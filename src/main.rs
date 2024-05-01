use std::io;

//imports metodo 1 sem a criação de arquivos extras
mod models {
    pub mod aluno;
}
mod core_config {
    pub mod config;
}
//imports metodo 2 sem o arquivo mod.rs
mod ui;
//imports metodo 3 com arquivo mod.rs
mod logica;
//metodo 4
#[path = "repositories/aluno_json.rs"]
mod aluno_json;
#[path = "repositories/aluno_mysql.rs"]
mod aluno_mysql;

fn main() -> io::Result<()> {
    
    ui::menu::menu();

    Ok(())
}