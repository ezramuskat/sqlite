use nom_sql::{CreateTableStatement, SelectStatement, SqlQuery};

pub fn execute_statement(query: SqlQuery) {
    println!("{}", query);
    match query {
        SqlQuery::CreateTable(statement) => {
            execute_create_table(statement);
        }
        SqlQuery::Select(statement) => {
            execute_select(statement);
        }
        _ => {
            println!("Unsupported statement type");
        }
    }
}

fn execute_create_table(statement: CreateTableStatement) {
    println!("CREATE TABLE statement executed");
}

fn execute_select(statement: SelectStatement) {
    println!("SELECT statement executed");
}
