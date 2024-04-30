use std::env;

pub fn get_json_db_alunos_path()->String{
    match env::var("JSON_DB_ALUNOS_PATH"){
        Ok(path) => path,
        Err(_) => String::from("db/alunos.json")
    }
}
pub fn get_mysql_db_alunos_path()->String{

    // "mysql://jose:jose@localhost:3306/rustdb"
    
    let user = match env::var("DATABASE_USER"){
        Ok(path) => path,
        Err(_) => String::from("root")
    };
    let password = match env::var("DATABASE_PASSWORD"){
        Ok(path) => path,
        Err(_) => String::from("")
    };
    let db = match env::var("DATABASE_DB"){
        Ok(path) => path,
        Err(_) => String::from("rustdb")
    };
    let host = match env::var("DATABASE_HOST"){
        Ok(path) => path,
        Err(_) => String::from("localhost")
    };
    let port = match env::var("DATABASE_PORT"){
        Ok(path) => path,
        Err(_) => String::from("3306")
    };

    format!("mysql://{}:{}@{}:{}/{}",password,user,host,port,db)
}