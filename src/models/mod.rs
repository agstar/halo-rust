use crate::Error;
use crate::utils::models::ModelExt;

pub mod cat;
pub mod user;

pub async fn sync_indexes() -> Result<(), Error> {
    user::User::sync_indexes().await?;
    cat::Cat::sync_indexes().await?;

    Ok(())
}
