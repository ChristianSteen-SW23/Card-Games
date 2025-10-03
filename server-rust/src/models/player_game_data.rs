use crate::models::Player;

pub trait PlayerGameData: std::fmt::Debug + Send + Sync {
    fn clone_box(&self) -> Box<dyn PlayerGameData + Send + Sync>;
}

impl Clone for Box<dyn PlayerGameData + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}