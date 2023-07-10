use crate::traits::post_service::{BasePost, Post, PostService, Tag};
use rbatis::rbatis::RBatis;
use screw_components::dyn_result::{DError, DResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn create_rbatis_post_service(rb: RBatis) -> Box<dyn PostService> {
    Box::new(RbatisPostService { rb })
}

impl_insert!(BasePost {}, "post");

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagDto {
    post_id: i64,
    id: i64,
    title: String,
}

impl Into<Tag> for TagDto {
    fn into(self) -> Tag {
        Tag {
            id: self.id,
            title: self.title,
        }
    }
}

impl Post {
    #[py_sql(
        "
        SELECT COUNT(1) \
        FROM post \
    "
    )]
    async fn count(rb: &RBatis) -> rbatis::Result<i64> {
        impled!()
    }
    #[py_sql(
        "
        SELECT \
            post.*, \
            author.slug AS author_slug, \
            author.first_name AS author_first_name, \
            author.last_name AS author_last_name \
        FROM post \
        JOIN author ON post.author_id = author.id \
        WHERE post.id = #{id} \
        LIMIT 1 \
    "
    )]
    async fn select_by_id(rb: &RBatis, id: &i64) -> rbatis::Result<Option<Post>> {
        impled!()
    }
    #[py_sql(
        "
        SELECT \
            post.*, \
            author.slug AS author_slug, \
            author.first_name AS author_first_name, \
            author.last_name AS author_last_name \
        FROM post \
        JOIN author ON post.author_id = author.id \
        WHERE post.slug = #{slug} \
        LIMIT 1 \
    "
    )]
    async fn select_by_slug(rb: &RBatis, slug: &String) -> rbatis::Result<Option<Post>> {
        impled!()
    }

    #[py_sql(
        "
        SELECT \
            post.*, \
            author.slug AS author_slug, \
            author.first_name AS author_first_name, \
            author.last_name AS author_last_name \
        FROM post \
        JOIN author ON post.author_id = author.id \
        LIMIT #{limit} \
        OFFSET #{offset} \
    "
    )]
    async fn select_posts(rb: &RBatis, limit: &i64, offset: &i64) -> rbatis::Result<Vec<Post>> {
        impled!()
    }

    #[py_sql(
        "
        SELECT \
            tag.id, \
            tag.title, \
            post_tag.post_id \
        FROM post_tag \
        JOIN tag ON tag.id = post_tag.tag_id \
        WHERE \
            post_tag.post_id IN (
                trim ',': for _,item in post_ids:
                    #{item},
                ) \
    "
    )]
    async fn select_tags_by_posts(rb: &RBatis, post_ids: Vec<i64>) -> rbatis::Result<Vec<TagDto>> {
        impled!()
    }

    fn apply_tags(&mut self, tags: &Vec<Tag>) {
        self.tags = Some(tags.clone());
    }
}

struct RbatisPostService {
    rb: RBatis,
}

#[async_trait]
impl PostService for RbatisPostService {
    async fn posts_count(&self) -> DResult<i64> {
        Ok(Post::count(&self.rb).await?)
    }
    async fn posts(&self, offset: &i64, limit: &i64) -> DResult<Vec<Post>> {
        let mut posts = Post::select_posts(&self.rb, limit, offset).await?;

        let post_ids = posts.iter().map(|post| post.id).collect();
        let grouped_tags: HashMap<i64, Vec<Tag>> = Post::select_tags_by_posts(&self.rb, post_ids)
            .await?
            .into_iter()
            .fold(HashMap::new(), |mut map, dto| {
                let key = dto.post_id;
                let tag = dto.into();
                map.entry(key).or_insert_with(Vec::new).push(tag);
                map
            });

        for post in posts.iter_mut() {
            match grouped_tags.get(&post.id) {
                Some(tags) => post.apply_tags(tags),
                None => {}
            }
        }

        Ok(posts)
    }

    async fn post_by_id(&self, id: &i64) -> DResult<Option<Post>> {
        let post_option = Post::select_by_id(&self.rb, id).await?;
        match post_option {
            None => Ok(None),
            Some(mut post) => {
                let post_tags = Post::select_tags_by_posts(&self.rb, vec![post.id])
                    .await?
                    .into_iter()
                    .map(|tag| tag.into())
                    .collect();
                post.apply_tags(&post_tags);

                Ok(Some(post))
            }
        }
    }

    async fn post_by_slug(&self, slug: &String) -> DResult<Option<Post>> {
        let post_option = Post::select_by_slug(&self.rb, slug).await?;

        match post_option {
            None => Ok(None),
            Some(mut post) => {
                let post_tags = Post::select_tags_by_posts(&self.rb, vec![post.id])
                    .await?
                    .into_iter()
                    .map(|tag| tag.into())
                    .collect();
                post.apply_tags(&post_tags);

                Ok(Some(post))
            }
        }
    }

    async fn create_post(&self, post: &BasePost) -> DResult<i64> {
        let insert_result = BasePost::insert(&mut self.rb.clone(), post).await?;
        let last_insert_id = insert_result
            .last_insert_id
            .as_i64()
            .ok_or::<DError>("wrond last_insert_id".into())?;
        Ok(last_insert_id)
    }
}
