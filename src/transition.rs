pub enum Transition {
    None,
    NewGame,
    GameOver(bool),
    ToStart,
}
