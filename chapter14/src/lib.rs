// Commenting Contained Items
//! # My Crate
//!
//! `chapter14` is a collection of utilities to make performing certain
//! calculations more convenient.

// Making Useful Documentation Comments
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = chapter14::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = chapter14::add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}
