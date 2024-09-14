use super::types::*;

#[derive(Default, Debug)]
pub struct ClassVersion {
    minor: U2,
    major: U2,
}

impl From<(U2, U2)> for ClassVersion {
    fn from(value: (U2, U2)) -> Self {
        let (minor, major) = value;
        Self { minor, major }
    }
}
