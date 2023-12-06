use crate::shared::types::Role;
pub struct Session {
    role: Role,
}

impl Session {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
    // pub fn can_remove_comment(&self) -> bool {
    //     self.role.can_remove_comment()
    // }
}
