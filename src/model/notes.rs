use std::fmt;

#[derive(Debug)]
pub struct Note {
    pub id: i32,
    pub name: String,
    pub content: String,
    pub created_at: String,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Note [id: {}, name: '{}', content: '{}', created_at: {}]",
            self.id, self.name, self.content, self.created_at
        )
    }
}