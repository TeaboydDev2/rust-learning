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
