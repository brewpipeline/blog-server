use crate::traits::comment_service::{BaseComment, Comment, CommentService};
use rbatis::rbatis::RBatis;
use screw_components::dyn_result::DResult;

pub fn create_rbatis_comment_service(rb: RBatis) -> Box<dyn CommentService> {
    Box::new(RbatisCommentService { rb })
}

impl_select!(Comment {select_all_by_post_id_with_limit_and_offset(post_id: &u64, limit: &u64, offset: &u64) => 
    "`WHERE post_id = #{post_id} LIMIT #{limit} OFFSET #{offset}`"}, "post_comment");
impl_select!(Comment {select_by_id(id: &u64) -> Option => 
        "`WHERE id = #{id} LIMIT 1`"}, "post_comment");

impl BaseComment {
    #[py_sql(
        "
        INSERT INTO post_comment
        (post_id, author_id, published, created_at, content)
        VALUES 
        (#{comment.post_id}, #{comment.author_id}, #{comment.published}, to_timestamp(#{comment.created_at}), #{comment.content})
        RETURNING id
    "
    )]
    async fn insert(rb: &RBatis, comment: &BaseComment) -> rbatis::Result<u64> {
        impled!()
    }
}

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
    #[py_sql(
        "
        UPDATE post_comment \
        SET \
            published = 0 \
        WHERE id = #{id}
    "
    )]
    async fn mark_deleted_by_id(rb: &RBatis, id: &u64) -> rbatis::Result<()> {
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
        let inserted_id = BaseComment::insert(&mut self.rb.clone(), comment).await?;
        Ok(inserted_id)
    }
    async fn comment_by_id(&self, id: &u64) -> DResult<Option<Comment>> {
        let comment = Comment::select_by_id(&mut self.rb.clone(), &id).await?;
        Ok(comment)
    }
    async fn mark_deleted_by_id(&self, id: &u64) -> DResult<()> {
        Comment::mark_deleted_by_id(&mut self.rb.clone(), &id).await?;
        Ok(())
    }
}
