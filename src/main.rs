

#[macro_use] extern crate postgres_mapper_derive;
extern crate postgres_mapper;
extern crate postgres;

use postgres_mapper::FromPostgresRow;

fn main() {
    println!("{}", User::sql_fields());
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "user")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub asdf: i32,
    pub email: Option<String>,
}

