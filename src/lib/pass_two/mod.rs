/**
 * Declare your modules here as pub
 */

/// This module is declared here so the compiler
/// konws that it exists and runs the tests, it's a private module
#[cfg(test)]
pub mod tests;
pub mod translator;
