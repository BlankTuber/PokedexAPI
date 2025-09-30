use crate::error::{ApiError, ApiResult};

pub struct CommonValidator;

impl CommonValidator {
    /// Validates that a string field is not empty after trimming
    pub fn validate_non_empty(value: &str, field_name: &str) -> ApiResult<()> {
        if value.trim().is_empty() {
            return Err(ApiError::BadRequest(format!(
                "{} cannot be empty",
                field_name
            )));
        }
        Ok(())
    }

    /// Validates that an optional string field is not empty if present
    pub fn validate_optional_non_empty(value: &Option<String>, field_name: &str) -> ApiResult<()> {
        if let Some(val) = value {
            Self::validate_non_empty(val, field_name)?;
        }
        Ok(())
    }

    /// Validates that a numeric value is within a range
    pub fn validate_range<T: PartialOrd + std::fmt::Display>(
        value: T,
        min: T,
        max: T,
        field_name: &str,
    ) -> ApiResult<()> {
        if value < min || value > max {
            return Err(ApiError::BadRequest(format!(
                "{} must be between {} and {}",
                field_name, min, max
            )));
        }
        Ok(())
    }

    /// Validates that an optional numeric value is within a range if present
    pub fn validate_optional_range<T: PartialOrd + std::fmt::Display + Copy>(
        value: &Option<T>,
        min: T,
        max: T,
        field_name: &str,
    ) -> ApiResult<()> {
        if let Some(val) = value {
            Self::validate_range(*val, min, max, field_name)?;
        }
        Ok(())
    }

    /// Validates that a value is positive
    pub fn validate_positive<T: PartialOrd + Default + std::fmt::Display>(
        value: T,
        field_name: &str,
    ) -> ApiResult<()> {
        if value <= T::default() {
            return Err(ApiError::BadRequest(format!(
                "{} must be positive",
                field_name
            )));
        }
        Ok(())
    }

    /// Validates that an identifier follows a specific pattern
    pub fn validate_identifier(value: &str, field_name: &str) -> ApiResult<()> {
        if value.trim().is_empty() {
            return Err(ApiError::BadRequest(format!(
                "{} cannot be empty",
                field_name
            )));
        }

        // Check if identifier contains only lowercase letters, numbers, hyphens, and underscores
        if !value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        {
            return Err(ApiError::BadRequest(format!(
                "{} must contain only lowercase letters, numbers, hyphens, and underscores",
                field_name
            )));
        }

        Ok(())
    }

    /// Validates that an optional identifier is valid if present
    pub fn validate_optional_identifier(value: &Option<String>, field_name: &str) -> ApiResult<()> {
        if let Some(val) = value {
            Self::validate_identifier(val, field_name)?;
        }
        Ok(())
    }

    /// Validates string length
    pub fn validate_length(value: &str, min: usize, max: usize, field_name: &str) -> ApiResult<()> {
        let len = value.len();
        if len < min || len > max {
            return Err(ApiError::BadRequest(format!(
                "{} must be between {} and {} characters",
                field_name, min, max
            )));
        }
        Ok(())
    }

    // Validates that an optional nested value is valid if present (for Option<Option<T>>)
    pub fn validate_optional_nested<T, F>(
        value: &Option<Option<T>>,
        validator: F,
        field_name: &str,
    ) -> ApiResult<()>
    where
        F: Fn(&T, &str) -> ApiResult<()>,
    {
        if let Some(Some(inner)) = value {
            validator(inner, field_name)?;
        }
        Ok(())
    }
}
