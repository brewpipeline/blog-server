use crate::traits::comment_service::{BaseComment, Comment, CommentService};
use rbatis::rbatis::RBatis;
use screw_components::dyn_result::{DError, DResult};

pub fn create_rbatis_comment_service(rb: RBatis) -> Box<dyn CommentService> {
    Box::new(RbatisCommentService { rb })
}

impl_insert!(BaseComment {}, "post_comment");

impl Comment {
    #[py_sql(
        "
        SELECT COUNT(1) \
        FROM post_comment \
        WHERE post_comment.post_id = #{post_id}
    "
    )]
    async fn count_by_post_id(rb: &RBatis, post_id: &u64) -> rbatis::Result<u64> {
        impled!()
    }
    #[py_sql(
        "
        SELECT \
            post_comment.*, \
            author.slug AS author_slug, \
            author.first_name AS author_first_name, \
            author.last_name AS author_last_name \
        FROM post_comment \
        JOIN author ON post_comment.author_id = author.id \
        WHERE post_comment.post_id = #{post_id} \
        LIMIT #{limit} \
        OFFSET #{offset} \
    "
    )]
    async fn select_all_by_post_id_with_limit_and_offset(
        rb: &RBatis,
        post_id: &u64,
        limit: &u64,
        offset: &u64,
    ) -> rbatis::Result<Vec<Comment>> {
        impled!()
    }
}

struct RbatisCommentService {
    rb: RBatis,
}

#[async_trait]
impl CommentService for RbatisCommentService {
    async fn comments_count_by_post_id(&self, post_id: &u64) -> DResult<u64> {
        Ok(Comment::count_by_post_id(&self.rb, post_id).await?)
    }
    async fn comments_by_post_id(
        &self,
        post_id: &u64,
        offset: &u64,
        limit: &u64,
    ) -> DResult<Vec<Comment>> {
        Ok(
            Comment::select_all_by_post_id_with_limit_and_offset(&self.rb, post_id, limit, offset)
                .await?,
        )
    }
    async fn create_comment(&self, comment: &BaseComment) -> DResult<u64> {
        let insert_result = BaseComment::insert(&mut self.rb.clone(), comment).await?;
        let last_insert_id = insert_result
            .last_insert_id
            .as_u64()
            .ok_or::<DError>("wrond last_insert_id".into())?;
        Ok(last_insert_id)
    }
}
