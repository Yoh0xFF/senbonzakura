use anyhow::Result;

/**
 * Base trait with common functionality for all visitors
 */
pub trait BaseVisitor {
    /**
     * The output type returned by visitor methodss
     */
    type Output;

    /**
     * The error type that can be returned by visitor methods
     */
    type Error;

    /**
     * Convert the visitor's error type to a Result
     */
    fn result<T>(&self, value: T) -> Result<T, Self::Error> {
        Ok(value)
    }
}
