pub struct User {
    pub id: u64,
    pub discord_id: u64,
    pub actions_allowed: bool,
    pub about: Option<String>,
    pub pronouns: Option<String>,
}
