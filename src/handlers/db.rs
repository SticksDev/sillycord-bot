use crate::structs::quote::Quote;
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

    pub async fn kv_set(&self, key: &str, value: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO kv_store (`key`, value) VALUES (?, ?) ON DUPLICATE KEY UPDATE value = ?",
            key,
            value,
            value
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn kv_get(&self, key: &str) -> Result<Option<String>, sqlx::Error> {
        let kv = sqlx::query!("SELECT * FROM kv_store WHERE `key` = ?", key)
            .fetch_optional(&self.db)
            .await?;

        match kv {
            Some(kv) => Ok(kv.value),
            None => Ok(None),
        }
    }

    pub async fn quote_create(&self, quote: Quote) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO quotes (user_id, username, quote, added_by) VALUES (?, ?, ?, ?)",
            quote.user_id,
            quote.username,
            quote.quote,
            quote.added_by
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn quote_get_random(&self) -> Result<Option<Quote>, sqlx::Error> {
        let quote = sqlx::query!("SELECT * FROM quotes ORDER BY RAND() LIMIT 1")
            .fetch_optional(&self.db)
            .await?;

        match quote {
            Some(quote) => Ok(Some(Quote {
                quote_id: quote.quote_id,
                user_id: quote.user_id,
                username: quote.username,
                quote: quote.quote,
                added_by: quote.added_by,
                added_at: quote.added_at.unwrap(),
            })),
            None => Ok(None),
        }
    }

    pub async fn quote_get_by_user_id(&self, user_id: u64) -> Result<Vec<Quote>, sqlx::Error> {
        let quote = sqlx::query!("SELECT * FROM quotes WHERE user_id = ?", user_id)
            .fetch_all(&self.db)
            .await?;

        let mut quotes = Vec::new();
        for q in quote {
            quotes.push(Quote {
                quote_id: q.quote_id,
                user_id: q.user_id,
                username: q.username,
                quote: q.quote,
                added_by: q.added_by,
                added_at: q.added_at.unwrap(),
            });
        }

        Ok(quotes)
    }
}
