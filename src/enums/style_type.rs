use serde::{Deserialize, Serialize};

/// Used to specify the kind of style of the application
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Debug)]
pub enum StyleType {
    Night,
    Day,
    DeepSea,
    MonAmour,
}

impl Default for StyleType {
    fn default() -> Self {
        Self::Night
    }
}
