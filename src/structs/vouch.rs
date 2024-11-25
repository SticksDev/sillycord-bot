use chrono::{DateTime, Utc};
use chrono_tz::US::Central;
use serenity::all::User;

#[derive(Debug, Clone)]
pub struct Vouch {
    pub user: User,
    pub vouched_by: User,
    pub vouch_time: DateTime<Utc>,
}

impl Vouch {
    pub fn new(user: User, vouched_by: User) -> Self {
        Self {
            user,
            vouched_by,
            vouch_time: Utc::now(),
        }
    }

    pub fn get_vouch_time(&self) -> String {
        self.vouch_time
            .with_timezone(&Central)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    }
}
