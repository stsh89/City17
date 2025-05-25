use uuid::Uuid;

pub struct GenerateUuidOperation;

pub enum UuidKind {
    V4,
    V7,
}

impl GenerateUuidOperation {
    pub fn execute(&self, kind: UuidKind) -> Uuid {
        match kind {
            UuidKind::V4 => Uuid::new_v4(),
            UuidKind::V7 => Uuid::now_v7(),
        }
    }
}
