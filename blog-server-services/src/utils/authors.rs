use std::collections::{HashMap, HashSet};

use screw_components::dyn_result::{DError, DResult};

use crate::traits::author_service::{Author, AuthorService};
use crate::traits::{Authored, FromAuthored};

pub async fn with_authors<T, E>(
    author_service: &dyn AuthorService,
    items: Vec<T>,
) -> DResult<Vec<E>>
where
    T: Authored,
    E: FromAuthored<T>,
{
    let authors_ids: HashSet<u64> = items.iter().map(Authored::author_id).collect();

    let authors: HashMap<u64, Author> = author_service
        .authors_by_ids(&authors_ids)
        .await?
        .into_iter()
        .map(|author| (author.id, author))
        .collect();

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
