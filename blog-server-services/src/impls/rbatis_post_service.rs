use crate::traits::post_service::{BasePost, Post, PostService, Tag};
use crate::utils::transliteration;
use rbatis::{rbatis::RBatis, rbdc::db::ExecResult};
use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub fn create_rbatis_post_service(rb: RBatis) -> Box<dyn PostService> {
    Box::new(RbatisPostService { rb })
}

impl_insert!(BasePost {}, "post");
impl_insert!(NewTag {}, "tag");
impl_select!(PostTag {select_by_post_id(post_id: &u64) =>
    "`WHERE post_id = #{post_id}`"});
impl_insert!(PostTag {}, "post_tag");

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NewTag {
    slug: String,
    title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagDto {
    post_id: u64,
    id: u64,
    slug: String,
    title: String,
}

impl Into<Tag> for TagDto {
    fn into(self) -> Tag {
        Tag {
            id: self.id,
            title: self.title,
            slug: self.slug,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostTag {
    post_id: u64,
    tag_id: u64,
}

impl PostTag {
    #[py_sql(
        "
        DELETE FROM post_tag \
        WHERE post_id = #{post_id} \
        AND \
        post_tag.tag_id IN (
            trim ',': for _,item in tag_ids:
                #{item},
            ) \
    "
    )]
    async fn delete_by_post_id_and_tag_ids(
        rb: &RBatis,
        post_id: u64,
        tag_ids: Vec<u64>,
    ) -> rbatis::Result<ExecResult> {
        impled!()
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
        SELECT COUNT(1) \
        FROM post \
        WHERE post.title ILIKE '%' || #{query} || '%' OR post.summary ILIKE '%' || #{query} || '%' OR post.content ILIKE '%' || #{query} || '%' \
    "
    )]
    async fn count_by_query(rb: &RBatis, query: &String) -> rbatis::Result<i64> {
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
    async fn select_by_id(rb: &RBatis, id: &u64) -> rbatis::Result<Option<Post>> {
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
            tag.slug, \
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
    async fn select_tags_by_posts(rb: &RBatis, post_ids: Vec<u64>) -> rbatis::Result<Vec<TagDto>> {
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
        WHERE post.title ILIKE '%' || #{query} || '%' OR post.summary ILIKE '%' || #{query} || '%' OR post.content ILIKE '%' || #{query} || '%' \
        LIMIT #{limit} \
        OFFSET #{offset} \
    "
    )]
    async fn select_all_by_query_with_limit_and_offset(
        rb: &RBatis,
        query: &String,
        limit: &i64,
        offset: &i64,
    ) -> rbatis::Result<Vec<Post>> {
        impled!()
    }

    fn apply_tags(&mut self, tags: Vec<Tag>) {
        self.tags = tags;
    }
}

struct RbatisPostService {
    rb: RBatis,
}

impl RbatisPostService {
    #[py_sql(
        "
        INSERT INTO post 
        (author_id,title,slug,summary,published,created_at,content) 
        VALUES 
        (#{post.author_id},#{post.title},#{post.slug},#{post.summary},#{post.published},to_timestamp(#{post.created_at}),#{post.content})
        RETURNING id
    "
    )]
    async fn insert_new_post(rb: &RBatis, post: &BasePost) -> rbatis::Result<u64> {
        impled!()
    }

    #[py_sql(
        "
        SELECT \
            tag.id, \
            tag.title, \
            tag.slug \
        FROM tag \
        WHERE \
            tag.slug IN (
                trim ',': for _,slug in slugs:
                    #{slug},
                ) \
    "
    )]
    async fn get_tags_by_slugs(rb: &RBatis, slugs: &Vec<String>) -> rbatis::Result<Vec<Tag>> {
        impled!()
    }

    async fn saturate_with_tags(&self, post_option: Option<Post>) -> DResult<Option<Post>> {
        match post_option {
            None => Ok(None),
            Some(mut post) => {
                let post_tags = Post::select_tags_by_posts(&self.rb, vec![post.id])
                    .await?
                    .into_iter()
                    .map(|tag| tag.into())
                    .collect();
                post.apply_tags(post_tags);
                Ok(Some(post))
            }
        }
    }

    async fn saturate_posts_with_tags(&self, mut posts: Vec<Post>) -> DResult<Vec<Post>> {
        if posts.is_empty() {
            return Ok(posts);
        }

        let post_ids = posts.iter().map(|post| post.id).collect();

        let mut grouped_tags: HashMap<u64, Vec<Tag>> =
            Post::select_tags_by_posts(&self.rb, post_ids)
                .await?
                .into_iter()
                .fold(HashMap::new(), |mut map, dto| {
                    let key = dto.post_id;
                    let tag = dto.into();
                    map.entry(key).or_insert_with(Vec::new).push(tag);
                    map
                });

        for post in posts.iter_mut() {
            match grouped_tags.remove(&post.id) {
                Some(tags) => post.apply_tags(tags),
                None => {}
            }
        }

        Ok(posts)
    }
}

#[async_trait]
impl PostService for RbatisPostService {
    async fn posts_count_by_query(&self, query: &String) -> DResult<i64> {
        Ok(Post::count_by_query(&self.rb, query).await?)
    }
    async fn posts_by_query(
        &self,
        query: &String,
        offset: &i64,
        limit: &i64,
    ) -> DResult<Vec<Post>> {
        let posts =
            Post::select_all_by_query_with_limit_and_offset(&self.rb, query, limit, offset).await?;
        RbatisPostService::saturate_posts_with_tags(&self, posts).await
    }

    async fn posts_count(&self) -> DResult<i64> {
        Ok(Post::count(&self.rb).await?)
    }
    async fn posts(&self, offset: &i64, limit: &i64) -> DResult<Vec<Post>> {
        let posts = Post::select_posts(&self.rb, limit, offset).await?;
        RbatisPostService::saturate_posts_with_tags(&self, posts).await
    }

    async fn post_by_id(&self, id: &u64) -> DResult<Option<Post>> {
        let post_option = Post::select_by_id(&self.rb, id).await?;
        RbatisPostService::saturate_with_tags(&self, post_option).await
    }

    async fn create_post(&self, post: &BasePost) -> DResult<u64> {
        let inserted_id = RbatisPostService::insert_new_post(&self.rb, post).await?;
        Ok(inserted_id)
    }

    async fn create_tags(&self, tag_titles: Vec<String>) -> DResult<Vec<Tag>> {
        if tag_titles.is_empty() {
            return Ok(vec![]);
        }

        let transliteration_results: Vec<transliteration::Transliteration> =
            transliteration::ru_to_latin(
                tag_titles.into_iter().map(|t| t.to_lowercase()).collect(),
            );
        let post_slugs: Vec<String> = transliteration_results
            .iter()
            .map(|r| r.transliterated.clone())
            .collect();
        let existing_by_slugs = RbatisPostService::get_tags_by_slugs(&self.rb, &post_slugs).await?;

        let existing_map =
            existing_by_slugs
                .iter()
                .fold(HashSet::new(), |mut set: HashSet<String>, tag| {
                    set.insert(tag.slug.clone());
                    set
                });
        let to_insert: Vec<NewTag> = transliteration_results
            .into_iter()
            .filter(|t| !existing_map.contains(&t.transliterated))
            .map(|tag| NewTag {
                slug: tag.transliterated,
                title: tag.original,
            })
            .collect();

        if to_insert.is_empty() {
            return Ok(existing_by_slugs);
        }

        NewTag::insert_batch(&mut self.rb.clone(), &to_insert, to_insert.len() as u64).await?;

        let all_tags = RbatisPostService::get_tags_by_slugs(&self.rb, &post_slugs).await?;
        Ok(all_tags)
    }

    async fn merge_post_tags(&self, post_id: &u64, tags: Vec<Tag>) -> DResult<()> {
        let new_tags_map: HashSet<u64> = tags.into_iter().fold(HashSet::new(), |mut set, tag| {
            set.insert(tag.id);
            set
        });

        let existing_tags_map = PostTag::select_by_post_id(&mut self.rb.clone(), post_id)
            .await?
            .into_iter()
            .fold(HashSet::new(), |mut set, post_tag| {
                set.insert(post_tag.tag_id);
                set
            });

        let to_insert: Vec<PostTag> = new_tags_map
            .iter()
            .filter(|new| !existing_tags_map.contains(new))
            .map(|to_insert| PostTag {
                post_id: *post_id,
                tag_id: *to_insert,
            })
            .collect();
        let to_delete: Vec<u64> = existing_tags_map
            .into_iter()
            .filter(|existing| !new_tags_map.contains(&existing))
            .collect();

        if !to_insert.is_empty() {
            PostTag::insert_batch(&mut self.rb.clone(), &to_insert, to_insert.len() as u64).await?;
        }
        if !to_delete.is_empty() {
            PostTag::delete_by_post_id_and_tag_ids(&self.rb, *post_id, to_delete).await?;
        }

        Ok(())
    }
}
