//! Spells floating-point numbers the way the language prints them: the shortest digits that read
//! back as the same value, in plain notation from `0.001` up to `10^7` and in scientific notation
//! like `1.5E10` outside it, always with a digit after the point. This is Java's
//! `Double.toString`, which the language has printed with from the start.

pub fn double(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let magnitude = value.abs();
    layout(&format!("{value:e}"), value == 0.0 || (1e-3..1e7).contains(&magnitude))
}

pub fn float(value: f32) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let magnitude = value.abs();
    layout(&format!("{value:e}"), value == 0.0 || (1e-3..1e7).contains(&magnitude))
}

/// Lays out Rust's shortest scientific form, like `-1.25e-3`, in plain or scientific notation.
fn layout(scientific: &str, plain: bool) -> String {
    let (mantissa, exponent) = scientific.split_once('e').expect("scientific notation");
    let exponent: i32 = exponent.parse().expect("exponent");
    let (sign, mantissa) = match mantissa.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", mantissa),
    };
    let digits: String = mantissa.chars().filter(|&c| c != '.').collect();

    if !plain {
        let (first, rest) = digits.split_at(1);
        let rest = if rest.is_empty() { "0" } else { rest };
        return format!("{sign}{first}.{rest}E{exponent}");
    }
    if exponent < 0 {
        let zeros = "0".repeat((-exponent - 1) as usize);
        return format!("{sign}0.{zeros}{digits}");
    }
    let point = exponent as usize + 1;
    if digits.len() <= point {
        let zeros = "0".repeat(point - digits.len());
        format!("{sign}{digits}{zeros}.0")
    } else {
        let (whole, fraction) = digits.split_at(point);
        format!("{sign}{whole}.{fraction}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles() {
        let cases = [
            (0.0, "0.0"),
            (-0.0, "-0.0"),
            (1.0, "1.0"),
            (3.5, "3.5"),
            (-2.25, "-2.25"),
            (0.1, "0.1"),
            (0.001, "0.001"),
            (0.0001, "1.0E-4"),
            (1234567.0, "1234567.0"),
            (1e7, "1.0E7"),
            (1.5e10, "1.5E10"),
            (1.8446744073709552e19, "1.8446744073709552E19"),
            (100.0, "100.0"),
            (f64::NAN, "NaN"),
            (f64::INFINITY, "Infinity"),
            (f64::NEG_INFINITY, "-Infinity"),
        ];
        for (value, text) in cases {
            assert_eq!(double(value), text, "{value:?}");
        }
    }

    #[test]
    fn floats() {
        assert_eq!(float(0.1), "0.1");
        assert_eq!(float(0.1 * 2.0), "0.2");
        assert_eq!(float(1e10), "1.0E10");
        assert_eq!(float(0.0), "0.0");
    }
}
