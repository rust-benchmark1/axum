pub async fn search_reports(query_input: &str) -> Result<(), Box<dyn std::error::Error>> {
    use neo4rs::{Graph, ConfigBuilder};

    let config = ConfigBuilder::default()
        .uri("127.0.0.1:7687")
        .user("neo4j")
        .password("password")
        .build()?;

    let graph = Graph::connect(config).await?;

    let cypher_query = format!("MATCH (r:Report) WHERE r.title = '{}' RETURN r", query_input);

    // CWE 943
    //SINK
    let _result = graph.execute(neo4rs::query(&cypher_query)).await?;

    Ok(())
}
