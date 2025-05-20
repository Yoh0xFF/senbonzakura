use crate::{
    ast::{PrimitiveType, Type},
    lexer::TokenType,
    Parser,
};

use super::internal_util::{eat_token, is_next_token_of_type};

pub(super) fn parse_type(parser: &mut Parser) -> Type {
    match parser.lookahead.token_type {
        TokenType::NumberTypeKeyword => {
            eat_token(parser, TokenType::NumberTypeKeyword);
            Type::Primitive(PrimitiveType::Number)
        }
        TokenType::StringTypeKeyword => {
            eat_token(parser, TokenType::StringTypeKeyword);
            Type::Primitive(PrimitiveType::String)
        }
        TokenType::BooleanTypeKeyword => {
            eat_token(parser, TokenType::BooleanTypeKeyword);
            Type::Primitive(PrimitiveType::Boolean)
        }
        TokenType::VoidTypeKeyword => {
            eat_token(parser, TokenType::VoidTypeKeyword);
            Type::Void
        }
        TokenType::Identifier => {
            // Handle class types or custom types
            let identifier_token = eat_token(parser, TokenType::Identifier);
            let type_name = &parser.source[identifier_token.i..identifier_token.j];

            // Check for generic type parameters
            if is_next_token_of_type(parser, TokenType::OpeningBracket) {
                eat_token(parser, TokenType::OpeningBracket);
                let mut type_args = vec![];

                loop {
                    type_args.push(parse_type(parser));

                    if !is_next_token_of_type(parser, TokenType::Comma) {
                        break;
                    }

                    eat_token(parser, TokenType::Comma);
                }

                eat_token(parser, TokenType::ClosingBracket);

                Type::Generic {
                    base: String::from(type_name),
                    type_args,
                }
            } else {
                Type::Class {
                    name: String::from(type_name),
                    super_class: None,
                }
            }
        }
        TokenType::OpeningBracket => {
            // Handle array types
            eat_token(parser, TokenType::OpeningBracket);
            let element_type = parse_type(parser);
            eat_token(parser, TokenType::ClosingBracket);

            Type::Array(Box::new(element_type))
        }
        _ => panic!(
            "Expected type annotation, found: {}",
            parser.lookahead.token_type
        ),
    }
}
