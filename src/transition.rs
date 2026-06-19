pub enum Transition {
    None,
    NewGame,
    GameOver { score: u32 },
    ToStart,
}
