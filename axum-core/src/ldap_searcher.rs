use ldap3::{LdapConn, Scope};

/// SINK CWE-90: Search LDAP using ldap3::LdapConn::search
/// Esta função atua como sink para LDAP injection
pub fn search_ldap_directory(query: String, base_dn: String) -> Result<(), Box<dyn std::error::Error>> {
    // Search LDAP directory using tainted query and base DN
    let mut ldap = LdapConn::new("ldap://localhost:389")?;
    //SINK
    ldap.simple_bind("cn=admin,dc=example,dc=com", "password")?;
    let (_rs, _res) = ldap.search(&base_dn, Scope::Subtree, &query, vec!["cn", "mail"])?.success()?;
    Ok(())
} 
