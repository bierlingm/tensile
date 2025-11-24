use crate::{
    error::{TensileError, TensileResult},
    models::VisionState,
};

pub struct StateMachine;

impl StateMachine {
    pub fn validate_transition(from: &VisionState, to: &VisionState) -> TensileResult<()> {
        let valid = match (from, to) {
            (VisionState::Conceived, VisionState::InProgress) => true,
            (VisionState::Conceived, VisionState::Achieved) => true,
            (VisionState::InProgress, VisionState::Blocked) => true,
            (VisionState::InProgress, VisionState::Reassessed) => true,
            (VisionState::InProgress, VisionState::Achieved) => true,
            (VisionState::Blocked, VisionState::InProgress) => true,
            (VisionState::Blocked, VisionState::Reassessed) => true,
            (VisionState::Reassessed, VisionState::InProgress) => true,
            (VisionState::Reassessed, VisionState::Achieved) => true,
            (a, b) if a == b => true,
            _ => false,
        };

        if valid {
            Ok(())
        } else {
            Err(TensileError::InvalidStateTransition(
                from.to_string(),
                to.to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(StateMachine::validate_transition(
            &VisionState::Conceived,
            &VisionState::InProgress
        )
        .is_ok());
        assert!(StateMachine::validate_transition(
            &VisionState::InProgress,
            &VisionState::Achieved
        )
        .is_ok());
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(StateMachine::validate_transition(
            &VisionState::Achieved,
            &VisionState::InProgress
        )
        .is_err());
    }
}
