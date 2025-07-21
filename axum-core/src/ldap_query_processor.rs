/// TRANSFORMER 1: Normalize LDAP query attributes
/// Appears to be a legitimate LDAP attribute normalization utility
pub fn normalize_ldap_attributes(raw_query: String) -> String {
    // TRANSFORMER 1: Normalize common LDAP attributes to uppercase
    let mut normalized = raw_query;
    let attribute_mappings = [
        ("uid", "UID"),
        ("cn", "CN"),
        ("sn", "SN"),
        ("mail", "MAIL"),
        ("ou", "OU"),
        ("dc", "DC"),
    ];
    
    for (lower, upper) in attribute_mappings.iter() {
        normalized = normalized.replace(lower, upper);
    }
    
    normalized
}

/// TRANSFORMER 2: Validate LDAP query structure
/// Appears to be a LDAP query structure validation utility
pub fn validate_ldap_structure(query: String) -> String {
    // TRANSFORMER 2: Basic LDAP query structure validation
    if !query.contains('(') || !query.contains(')') {
        // Add parentheses if missing
        if !query.starts_with('(') {
            return format!("({})", query);
        }
    }
    
    // Ensure proper closing
    let open_count = query.matches('(').count();
    let close_count = query.matches(')').count();
    
    if open_count > close_count {
        let missing = open_count - close_count;
        format!("{}{}", query, ")".repeat(missing))
    } else {
        query
    }
}

/// TRANSFORMER 3: Prepare LDAP query for search
/// Appears to be a LDAP query preparation utility
pub fn prepare_ldap_search_query(query: String) -> (String, String) {
    // TRANSFORMER 3: Prepare query and base DN for LDAP search
    let base_dn = if query.contains("dc=") {
        // Extract domain components for base DN
        let parts: Vec<&str> = query.split("dc=").collect();
        if parts.len() > 1 {
            format!("dc={}", parts[1].split(')').next().unwrap_or("example,dc=com"))
        } else {
            "dc=example,dc=com".to_string()
        }
    } else {
        "dc=example,dc=com".to_string()
    };
    
    // Clean up the query for search
    let search_query = if query.starts_with('(') && query.ends_with(')') {
        query
    } else {
        format!("({})", query)
    };
    
    (search_query, base_dn)
} 