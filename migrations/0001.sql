use rustdb;

CREATE TABLE aluno(
    nome VARCHAR(150) NOT NULL,
    matricula VARCHAR(50) NOT NULL,

    primary key (matricula)
);

create table nota(
    matricula varchar(50) not null,
    nota float not null,
    materia varchar(50) not null,
    foreign key (matricula) references aluno(matricula)
);
