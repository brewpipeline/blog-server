use std::collections::HashSet;

use crate::traits::author_service::{Author, AuthorService, BaseAuthor};
use rbatis::rbatis::RBatis;
use screw_components::dyn_result::{DError, DResult};

pub fn create_rbatis_author_service(rb: RBatis) -> Box<dyn AuthorService> {
    Box::new(RbatisAuthorService { rb })
}

impl_insert!(BaseAuthor {}, "author");
impl_select!(Author {select_by_id(id: &u64) -> Option => 
    "`WHERE id = #{id} LIMIT 1`"});
impl_select!(Author {select_by_slug(slug: &String) -> Option => 
    "`WHERE slug = #{slug} LIMIT 1`"});
impl_select!(Author {select_all_with_offset_and_limit(offset: &u64, limit: &u64) => 
    "`LIMIT #{limit} OFFSET #{offset}`"});
impl_select!(Author {select_all_by_query_with_offset_and_limit(query: &String, offset: &u64, limit: &u64) => 
    "`WHERE author.slug ILIKE '%' || #{query} || '%' OR author.first_name ILIKE '%' || #{query} || '%' OR author.middle_name ILIKE '%' || #{query} || '%' OR author.last_name ILIKE '%' || #{query} || '%' LIMIT #{limit} OFFSET #{offset}`"});

impl Author {
    #[py_sql(
        "
        SELECT COUNT(1) \
        FROM author \
    "
    )]
    async fn count(rb: &RBatis) -> rbatis::Result<u64> {
        impled!()
    }
    #[py_sql(
        "
        SELECT COUNT(1) \
        FROM author \
        WHERE author.slug ILIKE '%' || #{query} || '%' OR author.first_name ILIKE '%' || #{query} || '%' OR author.middle_name ILIKE '%' || #{query} || '%' OR author.last_name ILIKE '%' || #{query} || '%' \
    "
    )]
    async fn count_by_query(rb: &RBatis, query: &String) -> rbatis::Result<u64> {
        impled!()
    }
    #[py_sql(
        "
        SELECT \
            author.* \
        FROM author \
        WHERE \
            author.id IN (
                trim ',': for _,id in ids:
                    #{id},
                ) \
    "
    )]
    async fn select_by_ids(rb: &RBatis, ids: &HashSet<u64>) -> rbatis::Result<Vec<Author>> {
        impled!()
    }
    #[py_sql(
        "
        UPDATE author \
        SET \
            blocked = #{is_blocked} \
        WHERE id = #{id}
    "
    )]
    async fn set_blocked_by_id(rb: &RBatis, id: &u64, is_blocked: &u8) -> rbatis::Result<()> {
        impled!()
    }
}

struct RbatisAuthorService {
    rb: RBatis,
}

#[async_trait]
impl AuthorService for RbatisAuthorService {
    async fn authors_count_by_query(&self, query: &String) -> DResult<u64> {
        Ok(Author::count_by_query(&self.rb, query).await?)
    }
    async fn authors_by_query(
        &self,
        query: &String,
        offset: &u64,
        limit: &u64,
    ) -> DResult<Vec<Author>> {
        Ok(Author::select_all_by_query_with_offset_and_limit(
            &mut self.rb.clone(),
            query,
            offset,
            limit,
        )
        .await?)
    }
    async fn authors_count(&self) -> DResult<u64> {
        Ok(Author::count(&self.rb).await?)
    }
    async fn authors(&self, offset: &u64, limit: &u64) -> DResult<Vec<Author>> {
        Ok(Author::select_all_with_offset_and_limit(&mut self.rb.clone(), offset, limit).await?)
    }
    async fn authors_by_ids(&self, ids: &HashSet<u64>) -> DResult<Vec<Author>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        Ok(Author::select_by_ids(&self.rb, ids).await?)
    }
    async fn author_by_id(&self, id: &u64) -> DResult<Option<Author>> {
        Ok(Author::select_by_id(&mut self.rb.clone(), id).await?)
    }
    async fn author_by_slug(&self, slug: &String) -> DResult<Option<Author>> {
        Ok(Author::select_by_slug(&mut self.rb.clone(), slug).await?)
    }
    async fn create_author(&self, author: &BaseAuthor) -> DResult<u64> {
        let insert_result = BaseAuthor::insert(&mut self.rb.clone(), author).await?;
        let last_insert_id = insert_result
            .last_insert_id
            .as_u64()
            .ok_or::<DError>("wrond last_insert_id".into())?;
        Ok(last_insert_id)
    }
    async fn set_author_blocked_by_id(&self, id: &u64, is_blocked: &u8) -> DResult<()> {
        let _ = Author::set_blocked_by_id(&mut self.rb.clone(), &id, &is_blocked);
        Ok(())
    }
}
