use crate::ast::{Statement, StatementRef};
use crate::lexer::TokenType;
use crate::parser::parsers::{eat, root_expression, expression_statement, is_token, statement, variable_declaration_statement};
use crate::parser::Parser;

/**
 * WhileStatement
 *  : while '(' Expression ')' Statement ';'
 *  ;
 */
pub fn while_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::WhileKeyword);

    eat(parser, TokenType::OpeningParenthesis);
    let condition = root_expression(parser);
    eat(parser, TokenType::ClosingParenthesis);

    let body = statement(parser);

    Box::new(Statement::While { condition, body })
}

/**
 * DoWhileStatement
 *  : do Statement while '(' Expression ')' ';'
 *  ;
 */
pub fn do_while_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::DoKeyword);

    let body = statement(parser);

    eat(parser, TokenType::WhileKeyword);

    eat(parser, TokenType::OpeningParenthesis);
    let condition = root_expression(parser);
    eat(parser, TokenType::ClosingParenthesis);

    eat(parser, TokenType::StatementEnd);

    Box::new(Statement::DoWhile { body, condition })
}

/**
 * ForStatement
 *  : for '(' [InitExpression] ';' [Expression] ';' [Expression] ')' Statement
 *  ;
 */
pub fn for_statement(parser: &mut Parser) -> StatementRef {
    eat(parser, TokenType::ForKeyword);
    eat(parser, TokenType::OpeningParenthesis);

    let initializer = if is_token(parser, TokenType::StatementEnd) {
        None
    } else {
        Some(for_statement_init(parser))
    };
    eat(parser, TokenType::StatementEnd);

    let condition = if is_token(parser, TokenType::StatementEnd) {
        None
    } else {
        Some(root_expression(parser))
    };
    eat(parser, TokenType::StatementEnd);

    let increment = if is_token(parser, TokenType::ClosingParenthesis) {
        None
    } else {
        Some(root_expression(parser))
    };
    eat(parser, TokenType::ClosingParenthesis);

    let body = statement(parser);

    Box::new(Statement::For {
        initializer,
        condition,
        increment,
        body,
    })
}

/**
 * ForStatementInit
 *  : VariableDeclarationStatement
 *  | Expression
 *  ;
 */
pub fn for_statement_init(parser: &mut Parser) -> StatementRef {
    if is_token(parser, TokenType::LetKeyword) {
        return variable_declaration_statement(parser, false);
    }
    expression_statement(parser, false)
}
