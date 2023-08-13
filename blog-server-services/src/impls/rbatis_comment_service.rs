use crate::traits::comment_service::{BaseComment, Comment, CommentService};
use rbatis::rbatis::RBatis;
use screw_components::dyn_result::{DError, DResult};

pub fn create_rbatis_comment_service(rb: RBatis) -> Box<dyn CommentService> {
    Box::new(RbatisCommentService { rb })
}

impl_insert!(BaseComment {}, "post_comment");
impl_select!(Comment {select_all_by_post_id_with_limit_and_offset(post_id: &u64, limit: &u64, offset: &u64) => 
    "`WHERE post_id = #{post_id} LIMIT #{limit} OFFSET #{offset}`"}, "post_comment");

impl Comment {
    #[py_sql(
        "
        SELECT COUNT(1) \
        FROM post_comment \
        WHERE post_id = #{post_id}
    "
    )]
    async fn count_by_post_id(rb: &RBatis, post_id: &u64) -> rbatis::Result<u64> {
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
        Ok(Comment::select_all_by_post_id_with_limit_and_offset(
            &mut self.rb.clone(),
            post_id,
            limit,
            offset,
        )
        .await?)
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
