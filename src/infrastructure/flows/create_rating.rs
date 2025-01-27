use super::*;

use diesel::connection::Connection;

pub fn create_rating(
    connections: &sqlite::Connections,
    indexer: &mut dyn EntryIndexer,
    rate_entry: usecases::RateEntry,
) -> Result<(String, String)> {
    // Add new rating to existing entry
    let (rating_id, comment_id, entry, ratings) = {
        let connection = connections.exclusive()?;
        let mut prepare_err = None;
        connection
            .transaction::<_, diesel::result::Error, _>(|| {
                match usecases::prepare_new_rating(&*connection, rate_entry) {
                    Ok(storable) => {
                        let rating_id = storable.rating_id().to_owned();
                        let comment_id = storable.comment_id().to_owned();
                        let (entry, ratings) = usecases::store_new_rating(&*connection, storable)
                            .map_err(|err| {
                            warn!("Failed to store new rating for entry: {}", err);
                            diesel::result::Error::RollbackTransaction
                        })?;
                        Ok((rating_id, comment_id, entry, ratings))
                    }
                    Err(err) => {
                        prepare_err = Some(err);
                        Err(diesel::result::Error::RollbackTransaction)
                    }
                }
            })
            .map_err(|err| {
                if let Some(err) = prepare_err {
                    err
                } else {
                    RepoError::from(err).into()
                }
            })
    }?;

    // Reindex entry after adding the new rating
    // TODO: Move to a separate task/thread that doesn't delay this request
    if let Err(err) = usecases::index_entry(indexer, &entry, &ratings).and_then(|_| indexer.flush())
    {
        error!(
            "Failed to reindex entry {} after adding a new rating: {}",
            entry.id, err
        );
    }

    Ok((rating_id, comment_id))
}
