use crate::ast::{Statement, StatementList, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::statement_parse_conditional::if_statement;
use crate::parser::parsers::statement_parse_empty_and_expression::{
    empty_statement, expression_statement,
};
use crate::parser::parsers::statement_parse_loop::{
    do_while_statement, for_statement, while_statement,
};
use crate::parser::parsers::statement_parse_variable_declaration::variable_declaration_statement;
use crate::parser::parsers::utils::eat;
use crate::parser::Parser;
use crate::parser::parsers::statement_parse_function_declaration::{parse_function_declaration_statement, parse_return_statement};

/**
 * Main entry point
 *
 * Program
 *  : StatementList
 *  ;
 */
pub(super) fn program(parser: &mut Parser) -> StatementRef {
    let statement_list = statement_list(parser, None);
    Box::new(Statement::Program {
        body: statement_list,
    })
}

/**
 * BlockStatement
 *  : '{' OptStatementList '}'
 *  ;
 */
pub(super) fn block_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::OpeningBrace);

    let block = if parser.lookahead.token_type != TokenType::ClosingBrace {
        statement_list(parser, Some(TokenType::ClosingBrace))
    } else {
        vec![]
    };

    eat(parser, TokenType::ClosingBrace);

    Box::new(Statement::Block { body: block })
}

/**
 * StatementList
 *  : Statement
 *  | StatementList Statement
 *  ;
 */
pub(super) fn statement_list(
    parser: &mut Parser,
    stop_token_type: Option<TokenType>,
) -> StatementList {
    let mut statement_list: Vec<Statement> = vec![];

    while parser.lookahead.token_type != TokenType::End
        && parser.lookahead.token_type != stop_token_type.unwrap_or(TokenType::End)
    {
        let statement = statement(parser);
        statement_list.push(*statement);
    }

    statement_list
}

/**
 * Statement
 *  : ExpressionStatement
 *  | BlockStatement
 *  | EmptyStatement
 *  | VariableStatement
 *  | ConditionalStatement
 *  | IterationStatement
 *  | FunctionDeclarationStatement
 *  | ReturnStatement
 *  ;
 */
pub(super) fn statement(parser: &mut Parser) -> StatementRef {
    match parser.lookahead.token_type {
        TokenType::StatementEnd => empty_statement(parser),
        TokenType::OpeningBrace => block_statement(parser),
        TokenType::LetKeyword => variable_declaration_statement(parser, true),
        TokenType::IfKeyword => if_statement(parser),
        TokenType::WhileKeyword => while_statement(parser),
        TokenType::DoKeyword => do_while_statement(parser),
        TokenType::ForKeyword => for_statement(parser),
        TokenType::DefKeyword => parse_function_declaration_statement(parser),
        TokenType::ReturnKeyword => parse_return_statement(parser),
        _ => expression_statement(parser, true),
    }
}
