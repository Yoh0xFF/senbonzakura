use crate::{
    ast::{PrimitiveType, Type},
    lexer::TokenType,
    Parser,
};

use super::internal_util::{eat, is_token};

pub(super) fn parse_type(parser: &mut Parser) -> Type {
    match parser.lookahead.token_type {
        TokenType::NumberTypeKeyword => {
            eat(parser, TokenType::NumberTypeKeyword);
            Type::Primitive(PrimitiveType::Number)
        }
        TokenType::StringTypeKeyword => {
            eat(parser, TokenType::StringTypeKeyword);
            Type::Primitive(PrimitiveType::String)
        }
        TokenType::BooleanTypeKeyword => {
            eat(parser, TokenType::BooleanTypeKeyword);
            Type::Primitive(PrimitiveType::Boolean)
        }
        TokenType::VoidTypeKeyword => {
            eat(parser, TokenType::VoidTypeKeyword);
            Type::Void
        }
        TokenType::Identifier => {
            // Handle class types or custom types
            let identifier_token = eat(parser, TokenType::Identifier);
            let type_name = &parser.source[identifier_token.i..identifier_token.j];

            // Check for generic type parameters
            if is_token(parser, TokenType::OpeningBracket) {
                eat(parser, TokenType::OpeningBracket);
                let mut type_args = vec![];

                loop {
                    type_args.push(parse_type(parser));

                    if !is_token(parser, TokenType::Comma) {
                        break;
                    }

                    eat(parser, TokenType::Comma);
                }

                eat(parser, TokenType::ClosingBracket);

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
            eat(parser, TokenType::OpeningBracket);
            let element_type = parse_type(parser);
            eat(parser, TokenType::ClosingBracket);

            Type::Array(Box::new(element_type))
        }
        _ => panic!(
            "Expected type annotation, found: {}",
            parser.lookahead.token_type
        ),
    }
}
