use crate::traits::author_service::{Author, AuthorService};
use rbatis::rbatis::RBatis;
use rbs::to_value;
use screw_components::dyn_result::DResult;

pub fn create_rbatis_author_service(rb: RBatis) -> Box<dyn AuthorService> {
    Box::new(RbatisAuthorService { rb })
}

crud!(Author {});

struct RbatisAuthorService {
    rb: RBatis,
}

#[async_trait]
impl AuthorService for RbatisAuthorService {
    async fn authors_count(&self) -> DResult<i64> {
        let count: i64 = self
            .rb
            .query_decode("SELECT COUNT(*) FROM `author`", vec![])
            .await?;
        Ok(count)
    }
    async fn get_authors(&self, offset: &i64, limit: &i64) -> DResult<Vec<Author>> {
        let authors: Vec<Author> = self
            .rb
            .query_decode(
                "SELECT * FROM `author` LIMIT ? OFFSET ?",
                vec![to_value!(limit), to_value!(offset)],
            )
            .await?;
        Ok(authors)
    }
    async fn get_author_by_id(&self, id: &i64) -> DResult<Option<Author>> {
        Ok(Author::select_by_column(&mut self.rb.clone(), "id", id)
            .await?
            .pop())
    }
    async fn get_author_by_slug(&self, slug: &String) -> DResult<Option<Author>> {
        Ok(Author::select_by_column(&mut self.rb.clone(), "slug", slug)
            .await?
            .pop())
    }
    async fn create_author(&self, author: &Author) -> DResult<()> {
        let _ = Author::insert(&mut self.rb.clone(), author).await?;
        Ok(())
    }
}
