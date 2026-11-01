use crate::domain::todos::errors::TodoError;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TodoStatus {
    Done,
    InProgress,
    #[default]
    Todo,
}

impl TodoStatus {
    pub fn parse(raw: &str) -> Result<Self, TodoError> {
        match raw {
            "done"        => Ok(Self::Done),
            "in-progress" => Ok(Self::InProgress),
            "todo"        => Ok(Self::Todo),
            _             => Err(TodoError::InvalidStatus),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Done       => "done",
            Self::InProgress => "in-progress",
            Self::Todo       => "todo",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips() {
        for status in [TodoStatus::Done, TodoStatus::InProgress, TodoStatus::Todo] {
            assert_eq!(TodoStatus::parse(status.as_str()), Ok(status));
        }
        assert_eq!(TodoStatus::parse("blocked"), Err(TodoError::InvalidStatus));
    }
}
