pub trait MoveSet {
    fn get_relative_moves(&self) -> Vec<(i8,i8)>;
}

pub trait Movable {
    fn get_move_sets(&self) -> Vec<Box<dyn MoveSet>>;
}