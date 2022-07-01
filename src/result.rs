#[cfg(feature = "strict")]
/// Type alias for operation result
///
/// This is either ```boolean``` or ```Result<(), String>```
pub type OpResult = Result<(), String>;
#[cfg(not(feature = "strict"))]
/// Type alias for operation result
///
/// This is either ```boolean``` or ```Result<(), String>```
pub type OpResult = bool;

pub fn ok() -> OpResult {
    #[cfg(feature = "strict")]
    return Ok(());
    #[cfg(not(feature = "strict"))]
    return true;
}

#[allow(unused_variables)]
pub fn error(err: &str) -> OpResult {
    #[cfg(feature = "strict")]
    return Err(err.to_string());
    #[cfg(not(feature = "strict"))]
    return false;
}

pub fn is_true(result: OpResult) -> bool {
    #[cfg(feature = "strict")]
    return result.is_ok();
    #[cfg(not(feature = "strict"))]
    return result;
}
