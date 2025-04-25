use std::fmt::Write;

use super::SExpressionVisitor;
use crate::ast::Type;
use anyhow::Result;

pub(super) fn visit_type(visitor: &mut SExpressionVisitor, type_annotation: &Type) -> Result<()> {
    match type_annotation {
        Type::Primitive(primitive) => {
            write!(visitor.output, " {}", primitive)?;
        }
        Type::Array(element_type) => {
            visitor.begin_expr("array")?;
            visitor.write_space_or_newline()?;
            visit_type(visitor, element_type)?;
            visitor.end_expr()?;
        }
        Type::Function {
            params,
            return_type,
        } => {
            visitor.begin_expr("function")?;

            // Process parameter types
            visitor.write_space_or_newline()?;
            visitor.begin_expr("params")?;
            for param in params {
                visitor.write_space_or_newline()?;
                visit_type(visitor, param)?;
            }
            visitor.end_expr()?;

            // Process return type
            visitor.write_space_or_newline()?;
            visitor.begin_expr("return")?;
            visit_type(visitor, return_type)?;
            visitor.end_expr()?;

            visitor.end_expr()?;
        }
        Type::Class { name, super_class } => {
            visitor.begin_expr("class")?;
            write!(visitor.output, " {}", name)?;

            if let Some(super_name) = super_class {
                visitor.write_space_or_newline()?;
                write!(visitor.output, " {}", super_name)?;
            }

            visitor.end_expr()?;
        }
        Type::Generic { base, type_args } => {
            visitor.begin_expr("generic")?;
            write!(visitor.output, " \"{}\"", base)?;

            for type_arg in type_args {
                visitor.write_space_or_newline()?;
                visit_type(visitor, type_arg)?;
            }

            visitor.end_expr()?;
        }
        Type::Void => {
            write!(visitor.output, " \"void\"")?;
        }
    }

    Ok(())
}
