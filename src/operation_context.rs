pub struct OperationContext {
    pub subject_player: usize,
    pub subject_character: usize,
    pub target_player: usize,
    pub target_character: usize,
}

impl OperationContext {
    pub fn new(subject_player: usize, subject_character: usize, target_character: usize) -> Self {
        OperationContext {
            subject_player,
            subject_character,
            target_player: 1usize - subject_player,
            target_character,
        }
    }
}