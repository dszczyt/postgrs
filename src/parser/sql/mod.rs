pub mod keywords;
pub mod nodes;

// rustlex! SqlLexer {
//     let Select = "SELECT";
// }

extern crate pest;
use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/sql/sql.pest"]
pub struct SQLParser;

// pub fn parse_sql(query: String) -> Result<(), String> {
//     let parse_result = SQLParser::parse(Rule::dot, query);
//     Ok(())
// }