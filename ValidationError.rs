use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    Empty,
    InvalidFormat,
    InvalidDomain,
}

/// Sanitizes and validates a Shopify `shop` domain.
/// Returns the cleaned domain if valid, otherwise an error.
///
/// Examples of valid shops:
/// - "mystore.myshopify.com"
/// - "my-test-shop.myshopify.com"
pub fn sanitize_and_validate_shop_domain(input: &str) -> Result<String, ValidationError> {
    // 1. Sanitize input
    let mut shop = input.trim().to_lowercase();

    // Remove control characters or accidental whitespace
    shop.retain(|c| !c.is_control() && c != ' ');

    // 2. Basic checks
    if shop.is_empty() {
        return Err(ValidationError::Empty);
    }

    // 3. Validate format: must be something like <name>.myshopify.com
    // Shopify shop name can include letters, digits, and hyphens
    let re = Regex::new(r"^[a-z0-9][a-z0-9\-]*\.myshopify\.com$").unwrap();

    if !re.is_match(&shop) {
        return Err(ValidationError::InvalidFormat);
    }

    // 4. Optional: block certain domains (e.g., no `.myshopify.io` or custom domains)
    if !shop.ends_with(".myshopify.com") {
        return Err(ValidationError::InvalidDomain);
    }

    Ok(shop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_shop() {
        let shop = sanitize_and_validate_shop_domain("  My-Store.MyShopify.com  ").unwrap();
        assert_eq!(shop, "my-store.myshopify.com");
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(
            sanitize_and_validate_shop_domain("   ").unwrap_err(),
            ValidationError::Empty
        );
    }

    #[test]
    fn rejects_bad_format() {
        assert_eq!(
            sanitize_and_validate_shop_domain("evil.com").unwrap_err(),
            ValidationError::InvalidFormat
        );
        assert_eq!(
            sanitize_and_validate_shop_domain("store@myshopify.com").unwrap_err(),
            ValidationError::InvalidFormat
        );
    }

    #[test]
    fn rejects_custom_domain() {
        assert_eq!(
            sanitize_and_validate_shop_domain("store.myshopify.io").unwrap_err(),
            ValidationError::InvalidDomain
        );
    }
}
