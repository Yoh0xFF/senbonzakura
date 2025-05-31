use crate::{
    ast::{PrimitiveType, Type},
    lexer::TokenType,
    parser::{ParserError, ParserResult},
    Parser,
};

pub(super) fn parse_type(parser: &mut Parser) -> ParserResult<Type> {
    match parser.lookahead.token_type {
        TokenType::NumberTypeKeyword => {
            parser.eat_token(TokenType::NumberTypeKeyword)?;
            Ok(Type::Primitive(PrimitiveType::Number))
        }
        TokenType::StringTypeKeyword => {
            parser.eat_token(TokenType::StringTypeKeyword)?;
            Ok(Type::Primitive(PrimitiveType::String))
        }
        TokenType::BooleanTypeKeyword => {
            parser.eat_token(TokenType::BooleanTypeKeyword)?;
            Ok(Type::Primitive(PrimitiveType::Boolean))
        }
        TokenType::VoidTypeKeyword => {
            parser.eat_token(TokenType::VoidTypeKeyword)?;
            Ok(Type::Void)
        }
        TokenType::Identifier => {
            // Handle class types or custom types
            let identifier_token = parser.eat_token(TokenType::Identifier)?;
            let type_name = identifier_token.text(parser.source);

            // Check for generic type parameters
            if parser.is_next_token_of_type(TokenType::OpeningBracket) {
                parser.eat_token(TokenType::OpeningBracket)?;
                let mut type_args = vec![];

                loop {
                    let type_arg = parse_type(parser)?;
                    type_args.push(type_arg);

                    if !parser.is_next_token_of_type(TokenType::Comma) {
                        break;
                    }

                    parser.eat_token(TokenType::Comma)?;
                }

                parser.eat_token(TokenType::ClosingBracket)?;

                Ok(Type::Generic {
                    base: String::from(type_name),
                    type_args,
                })
            } else {
                Ok(Type::Class {
                    name: String::from(type_name),
                    super_class: None,
                })
            }
        }
        TokenType::OpeningBracket => {
            // Handle array types
            parser.eat_token(TokenType::OpeningBracket)?;
            let element_type = parse_type(parser)?;
            parser.eat_token(TokenType::ClosingBracket)?;

            Ok(Type::Array(Box::new(element_type)))
        }
        _ => Err(ParserError::TypeError {
            message: format!(
                "Expected type annotation, found: {}",
                parser.lookahead.token_type
            ),
        }),
    }
}
