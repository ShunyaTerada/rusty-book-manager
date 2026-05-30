use strum::{AsRefStr, EnumIter, EnumString};

#[derive(Debug, EnumString, AsRefStr, EnumIter, Default, PartialEq, Eq)]
pub enum Role {
    ADMIN,
    #[default]
    USER,
}
