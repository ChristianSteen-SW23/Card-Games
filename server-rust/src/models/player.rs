#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
