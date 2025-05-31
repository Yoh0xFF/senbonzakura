use crate::{
    ast::{ExpressionRef, Statement, StatementRef},
    lexer::TokenType,
    parser::{
        parsers::{
            expression_parse_primary::parse_identifier_expression,
            statement_parse_block::parse_block_statement,
        },
        ParserResult,
    },
    Parser,
};

///
/// ClassDeclaration
///  : class IdentifierExpression [ClassExtendsExpression] BlockStatement
///  ;
///
pub(super) fn parse_class_declaration(parser: &mut Parser) -> ParserResult<StatementRef> {
    parser.eat_token(TokenType::ClassKeyword)?;

    let name = parse_identifier_expression(parser)?;

    let super_class = if parser.is_next_token_of_type(TokenType::ExtendsKeyword) {
        let expression = parse_class_extends_expression(parser)?;
        Some(expression)
    } else {
        None
    };

    let body = parse_block_statement(parser)?;

    Ok(Box::new(Statement::ClassDeclaration {
        name,
        super_class,
        body,
    }))
}

///
/// ClassExtendsExpression
///  : extends IdentifierExpression
///  ;
///
fn parse_class_extends_expression(parser: &mut Parser) -> ParserResult<ExpressionRef> {
    parser.eat_token(TokenType::ExtendsKeyword)?;
    parse_identifier_expression(parser)
}
