use std::sync::Arc;

use crate::domain::repository::{
    adventurers::AdventurersRepository, guild_commanders::GuildCommandersRepository,
};

pub struct AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    adventurers_repository: Arc<T1>,
    guild_commanders_repository: Arc<T2>,
}

impl<T1, T2> AuthenticationUseCase<T1, T2>
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T1>, guild_commanders_repository: Arc<T2>) -> Self {
        Self {
            adventurers_repository,
            guild_commanders_repository,
        }
    }

    pub async fn adventurers_login(&self) {
        unimplemented!()
    }

    pub async fn adventurers_refresh_token(&self) {
        unimplemented!()
    }

    pub async fn guild_commanders_login(&self) {
        unimplemented!()
    }

    pub async fn guild_commanders_refresh_token(&self) {
        unimplemented!()
    }
}
