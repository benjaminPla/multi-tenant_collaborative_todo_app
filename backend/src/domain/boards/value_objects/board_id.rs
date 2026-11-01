use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BoardId(Uuid);

impl BoardId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl From<Uuid> for BoardId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}
