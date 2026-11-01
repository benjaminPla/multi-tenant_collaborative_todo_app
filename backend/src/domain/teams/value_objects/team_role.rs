use crate::domain::teams::errors::TeamError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TeamRole {
    Member,
    Owner,
}

impl TeamRole {
    pub fn parse(raw: &str) -> Result<Self, TeamError> {
        match raw {
            "member" => Ok(Self::Member),
            "owner"  => Ok(Self::Owner),
            _        => Err(TeamError::InvalidRole),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Member => "member",
            Self::Owner  => "owner",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_outranks_member() {
        assert!(TeamRole::Owner > TeamRole::Member);
    }

    #[test]
    fn round_trips() {
        for role in [TeamRole::Member, TeamRole::Owner] {
            assert_eq!(TeamRole::parse(role.as_str()), Ok(role));
        }
        assert_eq!(TeamRole::parse("admin"), Err(TeamError::InvalidRole));
    }
}
