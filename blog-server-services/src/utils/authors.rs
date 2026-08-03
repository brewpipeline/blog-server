use std::collections::{HashMap, HashSet};

use screw_components::dyn_result::{DError, DResult};

use crate::traits::author_service::{Author, AuthorService};
use crate::traits::{Authored, FromAuthored};

pub fn authors_ids<T: Authored>(items: &[T]) -> HashSet<u64> {
    items.iter().map(Authored::author_id).collect()
}

pub async fn authors_by_ids(
    author_service: &dyn AuthorService,
    authors_ids: &HashSet<u64>,
) -> DResult<HashMap<u64, Author>> {
    Ok(author_service
        .authors_by_ids(authors_ids)
        .await?
        .into_iter()
        .map(|author| (author.id, author))
        .collect())
}

pub async fn with_authors<T, E>(
    author_service: &dyn AuthorService,
    items: Vec<T>,
) -> DResult<Vec<E>>
where
    T: Authored,
    E: FromAuthored<T>,
{
    let authors = authors_by_ids(author_service, &authors_ids(&items)).await?;

    items
        .into_iter()
        .map(|item| {
            let author = authors
                .get(&item.author_id())
                .cloned()
                .ok_or::<DError>("wrong authors map".into())?;
            Ok(E::from_authored(item, author))
        })
        .collect()
}
