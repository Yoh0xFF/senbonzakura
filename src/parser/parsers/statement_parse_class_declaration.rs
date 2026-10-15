use crate::{
    ast::Statement,
    lexer::TokenType,
    parser::{parsers::statement_parse_block::parse_block_body, ParserResult},
    Parser,
};

///
/// ClassDeclaration
///  : class IDENTIFIER [ClassExtendsExpression] BlockStatement
///  ;
///
pub(super) fn parse_class_declaration(parser: &mut Parser) -> ParserResult<Box<Statement>> {
    parser.eat_token(TokenType::ClassKeyword)?;

    let name_token = parser.eat_token(TokenType::Identifier)?;
    let name = String::from(name_token.text(parser.source));

    let super_class = if parser.is_next_token_of_type(TokenType::ExtendsKeyword) {
        let super_class = parse_class_extends_expression(parser)?;
        Some(super_class)
    } else {
        None
    };

    let body = parse_block_body(parser)?;

    Ok(Box::new(Statement::ClassDeclaration {
        name,
        super_class,
        body,
    }))
}

///
/// ClassExtendsExpression
///  : extends IDENTIFIER
///  ;
///
fn parse_class_extends_expression(parser: &mut Parser) -> ParserResult<String> {
    parser.eat_token(TokenType::ExtendsKeyword)?;
    let name_token = parser.eat_token(TokenType::Identifier)?;
    Ok(String::from(name_token.text(parser.source)))
}
