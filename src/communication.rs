#[derive(Debug)]
pub enum DbMessage {
    Query(String),
    Quit,
}

#[derive(Debug)]
pub enum TuiMessage {
    QueryResponse(String),
    Failure(String),
}
