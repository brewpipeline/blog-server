use crate::traits::author_service::{Author, AuthorService};
use rbatis::rbatis::RBatis;
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
    async fn get_author_by_id(&self, id: &i64) -> DResult<Option<Author>> {
        Ok(
            Author::select_by_column(&mut self.rb.clone(), "id", id)
                .await?
                .pop(),
        )
    }
    async fn get_author_by_name(&self, authorname: &String) -> DResult<Option<Author>> {
        Ok(
            Author::select_by_column(&mut self.rb.clone(), "authorname", authorname)
                .await?
                .pop(),
        )
    }
    async fn create_author(&self, author: &Author) -> DResult<()> {
        let _ = Author::insert(&mut self.rb.clone(), author).await?;
        Ok(())
    }
}
