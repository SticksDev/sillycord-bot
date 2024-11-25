use crate::structs::user::User;
use sqlx::MySqlPool;

pub struct DatabaseController {
    db: MySqlPool,
}

impl DatabaseController {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }

    pub async fn get_user_by_discord_id(
        &self,
        discord_id: u64,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query!("SELECT * FROM users WHERE discord_id = ?", discord_id)
            .fetch_optional(&self.db)
            .await?;

        match user {
            Some(user) => Ok(Some(User {
                id: user.id,
                discord_id: user
                    .discord_id
                    .parse::<u64>()
                    .map_err(|_| sqlx::Error::Decode("Failed to parse discord_id".into()))?,
                actions_allowed: user.actions_allowed == Some(1),
                about: user.about,
                pronouns: user.pronouns,
            })),
            None => Ok(None),
        }
    }

    async fn get_user_by_id(&self, id: u64) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query!("SELECT * FROM users WHERE id = ?", id)
            .fetch_optional(&self.db)
            .await?;

        match user {
            Some(user) => Ok(Some(User {
                id: user.id,
                discord_id: user
                    .discord_id
                    .parse::<u64>()
                    .map_err(|_| sqlx::Error::Decode("Failed to parse discord_id".into()))?,
                actions_allowed: user.actions_allowed == Some(1),
                about: user.about,
                pronouns: user.pronouns,
            })),
            None => Ok(None),
        }
    }

    pub async fn create_user(&self, discord_id: u64) -> Result<User, sqlx::Error> {
        let user = sqlx::query!(
            "INSERT INTO users (discord_id) VALUES (?)",
            discord_id.to_string()
        )
        .execute(&self.db)
        .await?;

        Ok(User {
            id: user.last_insert_id(),
            discord_id,
            actions_allowed: true,
            about: None,
            pronouns: None,
        })
    }

    pub async fn update_user(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET actions_allowed = ?, about = ?, pronouns = ? WHERE id = ?",
            user.actions_allowed as i8,
            user.about,
            user.pronouns,
            user.id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn delete_user(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM users WHERE id = ?", user.id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn delete_user_by_discord_id(&self, discord_id: u64) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM users WHERE discord_id = ?", discord_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
