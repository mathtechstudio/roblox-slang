//! Translation validation
//!
//! This module provides validation functions for detecting missing translations,
//! conflicts, unused keys, and coverage analysis.

pub mod conflicts;
pub mod coverage;
pub mod missing;
pub mod unused;

/// Coverage information for a locale
#[derive(Debug, Clone)]
pub struct CoverageInfo {
    pub total_keys: usize,
    pub translated_keys: usize,
    pub missing_keys: Vec<String>,
    pub coverage_percent: f64,
}
