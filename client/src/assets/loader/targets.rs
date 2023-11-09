/*
    Any type of target that the loader can handle

    Might be changed to generic or user made later
*/
#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum TargetId {
    Sound,
    Sprite,
    Font,
}
