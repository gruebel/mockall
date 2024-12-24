//! Mockall should suppress clippy::ref_option_ref warnings in generated code.
#![warn(clippy::ref_option_ref)]

#[mockall::automock]
pub trait Store {
    fn find<'a>(&self, name: Option<&'a String>) -> bool;
}
