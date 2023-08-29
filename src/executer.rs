use nom_sql::SqlQuery;





pub fn execute_statement(query: SqlQuery) {
	println!("{}", query);
}