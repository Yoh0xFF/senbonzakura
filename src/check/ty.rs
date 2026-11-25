use crate::ast::PrimitiveKind;

/// The type of a value, as the [`Checker`](super) works it out. An [`ast::Type`](crate::ast::Type)
/// is a type as written, with a place in the source; this is what it means. It also covers what
/// no written type can say, like a number literal that has not settled on a type yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
    Primitive(PrimitiveKind),

    /// What `lower..upper` gives: the integers from `lower` up to, but not including, `upper`. The
    /// element is an integer [`Ty::Primitive`], or [`Ty::IntegerConstant`] while both bounds are.
    Range(Box<Ty>),

    /// What calling a function with several return types gives. Only a statement can use it.
    Multiple(Vec<Ty>),

    /// An integer literal, or arithmetic on only those. It takes the numeric type it goes into, if
    /// the literal fits it, and `int` when nothing asks for a type.
    IntegerConstant,

    /// Like [`Ty::IntegerConstant`] for float literals: `float` or `double`.
    FloatConstant,

    /// What calling a function without return types gives.
    Nothing,

    /// Accepts any value. Only the parameter of the built-in `print` has it.
    Any,

    /// The type of something whose problem was already reported. It goes into and accepts
    /// anything, so the one mistake is not reported again by everything that uses it.
    Error,
}

impl Ty {
    pub const BOOL: Ty = Ty::Primitive(PrimitiveKind::Bool);
    pub const CHAR: Ty = Ty::Primitive(PrimitiveKind::Char);
    pub const DOUBLE: Ty = Ty::Primitive(PrimitiveKind::Double);
    pub const INT: Ty = Ty::Primitive(PrimitiveKind::Int);
    pub const STRING: Ty = Ty::Primitive(PrimitiveKind::String);

    pub fn range(element: Ty) -> Ty {
        Ty::Range(Box::new(element))
    }

    pub(crate) fn is_constant(&self) -> bool {
        matches!(self, Ty::IntegerConstant | Ty::FloatConstant)
    }

    pub(crate) fn is_numeric(&self) -> bool {
        self.is_constant() || self.is_integer() || self.is_float()
    }

    pub(crate) fn is_integer(&self) -> bool {
        matches!(
            self,
            Ty::Primitive(
                PrimitiveKind::Byte
                    | PrimitiveKind::Short
                    | PrimitiveKind::Int
                    | PrimitiveKind::Size
                    | PrimitiveKind::Long
                    | PrimitiveKind::Ubyte
                    | PrimitiveKind::Ushort
                    | PrimitiveKind::Uint
                    | PrimitiveKind::Usize
                    | PrimitiveKind::Ulong
            )
        )
    }

    pub(crate) fn is_float(&self) -> bool {
        matches!(self, Ty::Primitive(PrimitiveKind::Float | PrimitiveKind::Double))
    }

    pub(crate) fn is_signed(&self) -> bool {
        matches!(
            self,
            Ty::Primitive(
                PrimitiveKind::Byte
                    | PrimitiveKind::Short
                    | PrimitiveKind::Int
                    | PrimitiveKind::Size
                    | PrimitiveKind::Long
                    | PrimitiveKind::Float
                    | PrimitiveKind::Double
            )
        )
    }

    /// How a diagnostic names the type, like `'int'` or `integer constant`.
    pub fn describe(&self) -> String {
        match self {
            Ty::Primitive(kind) => format!("'{}'", kind.spelling()),
            Ty::Range(element) if **element == Ty::IntegerConstant => {
                format!("a range of {}", Ty::INT.describe())
            }
            Ty::Range(element) => format!("a range of {}", element.describe()),
            Ty::Multiple(types) => format!("{} values", types.len()),
            Ty::IntegerConstant => "integer constant".to_string(),
            Ty::FloatConstant => "float constant".to_string(),
            Ty::Nothing => "no value".to_string(),
            Ty::Any => "a value".to_string(),
            Ty::Error => "an error".to_string(),
        }
    }
}

/// The smallest value of an integer type; `size` and `usize` are taken to be 64 bits wide.
pub(crate) fn min(kind: PrimitiveKind) -> i64 {
    match kind {
        PrimitiveKind::Byte => i8::MIN.into(),
        PrimitiveKind::Short => i16::MIN.into(),
        PrimitiveKind::Int => i32::MIN.into(),
        PrimitiveKind::Size | PrimitiveKind::Long => i64::MIN,
        _ => 0,
    }
}

/// The largest value of an integer type that fits in an `i64`; that is all of them but `ulong`
/// and `usize`, which no literal can go past anyway.
pub(crate) fn max(kind: PrimitiveKind) -> i64 {
    match kind {
        PrimitiveKind::Byte => i8::MAX.into(),
        PrimitiveKind::Short => i16::MAX.into(),
        PrimitiveKind::Int => i32::MAX.into(),
        PrimitiveKind::Ubyte => u8::MAX.into(),
        PrimitiveKind::Ushort => u16::MAX.into(),
        PrimitiveKind::Uint => u32::MAX.into(),
        _ => i64::MAX,
    }
}
