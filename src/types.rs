
#[derive(Debug, Clone)]
pub struct Chunk {
    pub checkpoint_id: u64,
    pub accounts: Vec<AccountInfo>,
}

#[derive(Debug, Clone)]
pub struct AccountInfo {
    pub id: String,
}

