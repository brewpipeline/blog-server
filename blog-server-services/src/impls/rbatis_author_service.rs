use crate::traits::author_service::{Author, AuthorService, BaseAuthor};
use rbatis::rbatis::RBatis;
use screw_components::dyn_result::{DError, DResult};

pub fn create_rbatis_author_service(rb: RBatis) -> Box<dyn AuthorService> {
    Box::new(RbatisAuthorService { rb })
}

impl_insert!(BaseAuthor {}, "author");
impl_select!(Author {select_by_id(id: &i64) -> Option => "`WHERE id = #{id} LIMIT 1`"});
impl_select!(Author {select_by_slug(slug: &String) -> Option => "`WHERE slug = #{slug} LIMIT 1`"});
impl_select!(Author {select_all_with_offset_and_limit(offset: &i64, limit: &i64) => "`LIMIT #{limit} OFFSET #{offset}`"});

impl Author {
    #[py_sql(
        "
        SELECT COUNT(1) \
        FROM author \
    "
    )]
    async fn count(rb: &RBatis) -> rbatis::Result<i64> {
        impled!()
    }
}

struct RbatisAuthorService {
    rb: RBatis,
}

#[async_trait]
impl AuthorService for RbatisAuthorService {
    async fn authors_count(&self) -> DResult<i64> {
        Ok(Author::count(&self.rb).await?)
    }
    async fn authors(&self, offset: &i64, limit: &i64) -> DResult<Vec<Author>> {
        Ok(Author::select_all_with_offset_and_limit(&mut self.rb.clone(), offset, limit).await?)
    }
    async fn author_by_id(&self, id: &i64) -> DResult<Option<Author>> {
        Ok(Author::select_by_id(&mut self.rb.clone(), id).await?)
    }
    async fn author_by_slug(&self, slug: &String) -> DResult<Option<Author>> {
        Ok(Author::select_by_slug(&mut self.rb.clone(), slug).await?)
    }
    async fn create_author(&self, author: &BaseAuthor) -> DResult<i64> {
        let insert_result = BaseAuthor::insert(&mut self.rb.clone(), author).await?;
        let last_insert_id = insert_result
            .last_insert_id
            .as_i64()
            .ok_or::<DError>("wrond last_insert_id".into())?;
        Ok(last_insert_id)
    }
}
