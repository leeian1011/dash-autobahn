use crate::dsh::lane::Lane;

pub enum ActionCode {
    Success,
    LaneDoesNotExist,
    LaneAlreadyExist,
    EmptyDasher,
}

