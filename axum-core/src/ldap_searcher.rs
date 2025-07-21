use ldap3::{LdapConn, Scope};
use crate::file_utils::receive_ldap_query;

//CWE-90: Search LDAP using ldap3::LdapConn::search

pub async fn search_ldap_directory(query: String, base_dn: String) -> Result<(), Box<dyn std::error::Error>> {
    // Search LDAP directory using tainted query and base DN
    let mut ldap = LdapConn::new("ldap://localhost:389")?;
    let tainted_dn = receive_ldap_query().await?;
    //SINK
    ldap.simple_bind(&tainted_dn, "password")?;
    let (_rs, _res) = ldap.search(&base_dn, Scope::Subtree, &query, vec!["cn", "mail"])?
        .success()?;
    Ok(())
} 
