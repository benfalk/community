// Bring into scope our application error
pub use crate::error::Error;

// Application specific Result
pub type Result<T> = std::result::Result<T, Error>;

// NewType wrapper for easier type conversions
pub struct W<T>(pub T);

// Context is used in a lot of places
pub use crate::Context;
