#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Player {
    One,
    Two,
    None,
}

impl Player {
    pub fn is_none(player: Player) -> bool {
        player == Player::None 
    }

    pub fn _is_one(player: Player) -> bool {
        player == Player::None 
    }

    fn _is_two(player: Player) -> bool {
        player == Player::None 
    }


}
