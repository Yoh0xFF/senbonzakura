use crate::{
    ast::{PrimitiveType, Type},
    lexer::TokenType,
    Parser,
};

pub(super) fn parse_type(parser: &mut Parser) -> Type {
    match parser.lookahead.token_type {
        TokenType::NumberTypeKeyword => {
            parser.eat_token(TokenType::NumberTypeKeyword);
            Type::Primitive(PrimitiveType::Number)
        }
        TokenType::StringTypeKeyword => {
            parser.eat_token(TokenType::StringTypeKeyword);
            Type::Primitive(PrimitiveType::String)
        }
        TokenType::BooleanTypeKeyword => {
            parser.eat_token(TokenType::BooleanTypeKeyword);
            Type::Primitive(PrimitiveType::Boolean)
        }
        TokenType::VoidTypeKeyword => {
            parser.eat_token(TokenType::VoidTypeKeyword);
            Type::Void
        }
        TokenType::Identifier => {
            // Handle class types or custom types
            let identifier_token = parser.eat_token(TokenType::Identifier);
            let type_name = identifier_token.text(parser.source);

            // Check for generic type parameters
            if parser.is_next_token_of_type(TokenType::OpeningBracket) {
                parser.eat_token(TokenType::OpeningBracket);
                let mut type_args = vec![];

                loop {
                    type_args.push(parse_type(parser));

                    if !parser.is_next_token_of_type(TokenType::Comma) {
                        break;
                    }

                    parser.eat_token(TokenType::Comma);
                }

                parser.eat_token(TokenType::ClosingBracket);

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
            parser.eat_token(TokenType::OpeningBracket);
            let element_type = parse_type(parser);
            parser.eat_token(TokenType::ClosingBracket);

            Type::Array(Box::new(element_type))
        }
        _ => panic!(
            "Expected type annotation, found: {}",
            parser.lookahead.token_type
        ),
    }
}
