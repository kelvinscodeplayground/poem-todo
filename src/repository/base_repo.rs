use anyhow::Result;

use crate::entity::user::User;

pub trait BaseRepo {
    async fn create_user(&self, user: &User) -> Result<()>;
}
