use crate::objects::Player;
#[derive(Debug, Clone)]
pub struct Players(Vec<Player>);

impl Players {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get(&self, key: &str) -> Option<&Player> {
        self.0.iter().find(|player| player.id == key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Player> {
        self.0.iter_mut().find(|player| player.id == key)
    }

    pub fn get_all(&self) -> &Vec<Player> {
        &self.0
    }

    pub fn get_next(&self, key: &str) -> Option<&Player> {
        self.0.iter().position(|player| player.id == key).and_then(|idx| self.0.get((idx + 1) % self.0.len()))
    }

    pub fn get_mut_all(&mut self) -> &mut Vec<Player> {
        &mut self.0
    }
    
    pub fn add(&mut self, player: Player){
        self.0.push(player);
    }

    pub fn remove(&mut self, key: &str) {
        let Some(idx) = self.0.iter().position(|player| player.id == key) else { return; };
        self.0.remove(idx);
    }
}
