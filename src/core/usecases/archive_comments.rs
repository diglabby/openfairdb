use crate::core::prelude::*;

pub fn archive_comments<D: Db>(db: &D, user_email: &str, ids: &[&str]) -> Result<()> {
    info!("Archiving {} comments", ids.len());
    // TODO: Pass an authentication token with user id and role to
    // check if the user is authorized to perform this use case
    let users = db.get_users_by_email(user_email)?;
    if let Some(user) = users.first() {
        if user.role >= Role::Scout {
            let archived = Timestamp::now();
            db.archive_comments(ids, archived)?;
            return Ok(());
        }
    }
    Err(ParameterError::Forbidden.into())
}
