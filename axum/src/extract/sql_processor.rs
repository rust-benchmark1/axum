use tokio_postgres::{Client, NoTls};

/// TRANSFORMER 1: Normalize SQL query structure
/// Appears to be a legitimate SQL query normalization utility
pub fn normalize_sql_query(raw_query: String) -> String {
    // TRANSFORMER 1: Normalize common SQL patterns and remove extra whitespace
    let mut normalized = raw_query.trim().to_string();
    
    // Replace common SQL abbreviations
    normalized = normalized.replace("SELECT *", "SELECT ALL");
    normalized = normalized.replace("WHERE 1=1", "WHERE TRUE");
    normalized = normalized.replace("WHERE 1=0", "WHERE FALSE");
    
    // Remove extra whitespace around operators
    normalized = normalized.replace(" = ", "=");
    normalized = normalized.replace(" != ", "!=");
    normalized = normalized.replace(" < ", "<");
    normalized = normalized.replace(" > ", ">");
    normalized = normalized.replace(" AND ", " AND ");
    normalized = normalized.replace(" OR ", " OR ");
    
    normalized
}

/// TRANSFORMER 2: Validate SQL query syntax
/// Appears to be a SQL query syntax validation utility
pub fn validate_sql_syntax(query: String) -> String {
    // TRANSFORMER 2: Basic SQL syntax validation and cleanup
    let mut validated = query;
    
    // Ensure proper SELECT syntax
    if !validated.to_uppercase().starts_with("SELECT") && 
       !validated.to_uppercase().starts_with("INSERT") &&
       !validated.to_uppercase().starts_with("UPDATE") &&
       !validated.to_uppercase().starts_with("DELETE") {
        validated = format!("SELECT * FROM users WHERE {}", validated);
    }
    
    // Ensure proper table reference
    if validated.to_uppercase().contains("FROM") && !validated.to_uppercase().contains("FROM USERS") {
        validated = validated.replace("FROM", "FROM users");
    }
    
    validated
}

/// TRANSFORMER 3: Process SQL query for execution
/// Appears to be a SQL query processing utility
pub fn process_sql_query(raw_query: String) -> String {
    // TRANSFORMER 3: Final processing before SQL execution
    let normalized = normalize_sql_query(raw_query);
    let validated = validate_sql_syntax(normalized);
    
    // Add semicolon if missing
    if !validated.trim().ends_with(';') {
        format!("{};", validated)
    } else {
        validated
    }
}

//CWE-89: Execute SQL batch using tokio_postgres::Client::batch_execute
// This function acts as a sink for SQL injection vulnerability testing
pub fn execute_sql_batch(query: String) -> Result<(), Box<dyn std::error::Error>> {
    
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    
    rt.block_on(async {
        // Connect to PostgreSQL database
        let (client, connection) = tokio_postgres::connect(
            "host=localhost user=postgres password=password dbname=testdb",
            NoTls,
        ).await?;
        
        // Spawn the connection to run it in the background
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        
        //SINK
        client.batch_execute(&query).await?;
        
        Ok::<(), Box<dyn std::error::Error>>(())
    })
} 
