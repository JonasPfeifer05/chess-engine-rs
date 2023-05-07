use dyn_clone::DynClone;

/// A descriptor for a movement pattern like horizontal LeftEight or UpOne and so on
pub trait MoveSet: DynClone {
    fn get_relative_moves(&self) -> Vec<(i8,i8)>;
}
dyn_clone::clone_trait_object!(MoveSet);
