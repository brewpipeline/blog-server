use crate::traits::post_service::{BasePost, Post, PostService, PostsQuery, PostsQueryAnswer, Tag};
use crate::utils::{string_filter, transliteration};
use rbatis::executor::RBatisTxExecutorGuard;
use rbatis::{rbatis::RBatis, rbdc::db::ExecResult};
use rbs::{Value, value};
use screw_components::dyn_result::DResult;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub fn create_rbatis_post_service(rb: RBatis) -> Arc<dyn PostService> {
    Arc::new(RbatisPostService { rb })
}

impl_insert!(BasePost {}, "post");
impl_insert!(NewTag {}, "tag");
impl_insert!(PostTag {}, "post_tag");

impl_select!(Tag {select_by_id(id: &u64) -> Option =>
    "`WHERE id = #{id} LIMIT 1`"});
impl_select!(PostTag {select_all_by_post_id(post_id: &u64) =>
    "`WHERE post_id = #{post_id}`"});

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
    #[py_sql(
        "
        DELETE FROM post_tag \
        WHERE post_id = #{post_id} \
    "
    )]
    async fn delete_by_post_id(
        rb: &mut RBatisTxExecutorGuard,
        post_id: &u64,
    ) -> rbatis::Result<ExecResult> {
        impled!()
    }
}

impl Post {
    #[py_sql(
        "
        SELECT \
            post.* \
        FROM post \
        WHERE post.id = #{id} \
        LIMIT 1 \
    "
    )]
    async fn single_by_id(rb: &RBatis, id: &u64) -> rbatis::Result<Option<Post>> {
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
            post.* \
        FROM post \
        JOIN post_tag ON post.id = post_tag.post_id \
        WHERE \
            post.recommended = 1 \
            AND post.publish_type = 1 \
            AND post.id <> #{post_id} \
            AND post_tag.tag_id IN (
                SELECT tag_id FROM post_tag WHERE post_id = #{post_id}
            ) \
        ORDER BY random() \
        LIMIT 1 \
    "
    )]
    async fn random_recommended_post(rb: &RBatis, post_id: &u64) -> rbatis::Result<Option<Post>> {
        impled!()
    }

    #[py_sql(
        "
        UPDATE post \
        SET recommended = #{recommended} \
        WHERE id = #{id} \
    "
    )]
    async fn set_recommended_by_id(
        rb: &RBatis,
        id: &u64,
        recommended: &u8,
    ) -> rbatis::Result<ExecResult> {
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
        (author_id,title,slug,summary,publish_type,created_at,content,plain_text_content,image_url)
        VALUES 
        (#{post.author_id},#{post.title},#{post.slug},#{post.summary},#{post.publish_type},to_timestamp(#{post.created_at}),#{post.content},#{post.plain_text_content},#{post.image_url})
        RETURNING id
    "
    )]
    async fn insert_new_post(rb: &RBatis, post: &BasePost) -> rbatis::Result<u64> {
        impled!()
    }

    #[py_sql(
        "
        UPDATE post \
        SET \
            title = #{post_data.title}, \
            slug = #{post_data.slug}, \
            summary = #{post_data.summary}, \
            publish_type = #{post_data.publish_type},
            if update_created_at:
                created_at = to_timestamp(#{post_data.created_at}),
            content = #{post_data.content}, \
            plain_text_content = #{post_data.plain_text_content}, \
            image_url = #{post_data.image_url} \
        WHERE id = #{post_id} \
        RETURNING id
    "
    )]
    async fn update_post_by_id(
        rb: &RBatis,
        post_id: &u64,
        post_data: &BasePost,
        update_created_at: &bool,
    ) -> rbatis::Result<u64> {
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
            tag.title IN (
                trim ',': for _,title in titles:
                    #{title},
                ) \
    "
    )]
    async fn get_tags_by_titles(rb: &RBatis, titles: &Vec<String>) -> rbatis::Result<Vec<Tag>> {
        impled!()
    }
    #[py_sql(
        "
        DELETE FROM post \
        WHERE post.id = #{id} \
    "
    )]
    async fn delete_post_by_id(
        rb: &mut RBatisTxExecutorGuard,
        id: &u64,
    ) -> rbatis::Result<ExecResult> {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct PostAndTotalCount {
    #[serde(flatten)]
    pub origin: Post,
    pub total_count: u64,
}

#[async_trait]
impl PostService for RbatisPostService {
    async fn posts<'q, 'a, 't, 'p, 'o, 'l>(
        &self,
        query: PostsQuery<'q, 'a, 't, 'p, 'o, 'l>,
    ) -> DResult<PostsQueryAnswer> {
        let mut args: Vec<Value> = vec![];
        let query = vec![
            {
                let mut select_parts = vec!["post.*"];
                if let Some(_) = query.search_query {
                    select_parts.push("ts_rank_cd(textsearch, query) AS rank");
                }
                select_parts.push("COUNT(*) OVER() AS total_count");
                Some(format!("SELECT {}", select_parts.join(", ")))
            },
            {
                let mut from_parts = vec!["post"];
                if let Some(search_query) = query.search_query {
                    from_parts.push("plainto_tsquery('russian', LOWER(?)) query");
                    args.push(value!(search_query));
                    from_parts.push("to_tsvector('russian', LOWER(post.title || ' ' || post.summary || ' ' || post.plain_text_content)) textsearch");
                }
                Some(format!("FROM {}", from_parts.join(", ")))
            },
            {
                let mut join_parts = vec![];
                if let Some(_) = query.tag_id {
                    join_parts.push("post_tag ON post.id = post_tag.post_id");
                }
                if join_parts.is_empty() {
                    None
                } else {
                    Some(format!("JOIN {}", join_parts.join(", ")))
                }
            },
            {
                let mut where_parts = vec![];
                if let Some(_) = query.search_query {
                    where_parts.push("textsearch @@ query");
                }
                if let Some(author_id) = query.author_id {
                    where_parts.push("author_id = ?");
                    args.push(value!(author_id));
                }
                if let Some(tag_id) = query.tag_id {
                    where_parts.push("post_tag.tag_id = ?");
                    args.push(value!(tag_id));
                }
                if let Some(publish_type) = query.publish_type {
                    where_parts.push("publish_type = ?");
                    args.push(value!(publish_type));
                }
                if where_parts.is_empty() {
                    None
                } else {
                    Some(format!("WHERE {}", where_parts.join(" AND ")))
                }
            },
            {
                let mut order_by_parts = vec![];
                if let Some(_) = query.search_query {
                    order_by_parts.push("rank");
                }
                order_by_parts.push("post.id DESC");
                Some(format!("ORDER BY {}", order_by_parts.join(", ")))
            },
            {
                args.push(value!(query.limit));
                Some("LIMIT ?".to_string())
            },
            {
                args.push(value!(query.offset));
                Some("OFFSET ?".to_string())
            }
        ]
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<String>>()
            .join(" ");

        let posts_with_total_count: Vec<PostAndTotalCount> =
            self.rb.query_decode(query.as_str(), args).await?;

        let total_count = posts_with_total_count
            .first()
            .map(|p| p.total_count)
            .unwrap_or(0);
        let posts = posts_with_total_count
            .into_iter()
            .map(|p| p.origin)
            .collect();

        let posts_with_tags = RbatisPostService::saturate_posts_with_tags(&self, posts).await?;

        Ok(PostsQueryAnswer {
            total_count,
            posts: posts_with_tags,
        })
    }

    async fn post_by_id(&self, id: &u64) -> DResult<Option<Post>> {
        let post_option = Post::single_by_id(&self.rb, id).await?;
        RbatisPostService::saturate_with_tags(&self, post_option).await
    }

    async fn create_post(&self, post: &BasePost) -> DResult<u64> {
        let inserted_id = RbatisPostService::insert_new_post(&self.rb, post).await?;
        Ok(inserted_id)
    }

    async fn update_post_by_id(
        &self,
        id: &u64,
        post_data: &BasePost,
        update_created_at: &bool,
    ) -> DResult<()> {
        RbatisPostService::update_post_by_id(&self.rb, id, post_data, update_created_at).await?;
        Ok(())
    }

    async fn delete_post_by_id(&self, id: &u64) -> DResult<()> {
        let tx = self.rb.acquire_begin().await?;
        let mut tx = tx.defer_async(|tx| async move {
            if !tx.done() {
                let _ = tx.rollback().await;
            }
        });
        PostTag::delete_by_post_id(&mut tx, id).await?;
        RbatisPostService::delete_post_by_id(&mut tx, id).await?;
        tx.commit().await?;
        Ok(())
    }

    async fn random_recommended_post(&self, post_id: &u64) -> DResult<Option<Post>> {
        let post_option = Post::random_recommended_post(&self.rb, post_id).await?;
        RbatisPostService::saturate_with_tags(&self, post_option).await
    }

    async fn set_post_recommended_by_id(&self, id: &u64, recommended: &u8) -> DResult<()> {
        Post::set_recommended_by_id(&self.rb, id, recommended).await?;
        Ok(())
    }

    async fn tag_by_id(&self, id: &u64) -> DResult<Option<Tag>> {
        let tag = Tag::select_by_id(&mut self.rb.clone(), id).await?;
        Ok(tag)
    }
    async fn create_tags(&self, tag_titles: Vec<String>) -> DResult<Vec<Tag>> {
        if tag_titles.is_empty() {
            return Ok(vec![]);
        }
        let tag_titles: Vec<String> = tag_titles
            .into_iter()
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();
        let search_titles = tag_titles.clone();

        let existing_by_titles =
            RbatisPostService::get_tags_by_titles(&self.rb, &search_titles).await?;

        let existing_map =
            existing_by_titles
                .iter()
                .fold(HashSet::new(), |mut set: HashSet<String>, tag| {
                    set.insert(tag.title.clone());
                    set
                });

        let fresh_tags: Vec<NewTag> =
            transliteration::ru_to_latin(tag_titles, transliteration::TranslitOption::ToLowerCase)
                .into_iter()
                .map(|r| NewTag {
                    slug: string_filter::remove_non_latin_or_number_chars(&r.transliterated),
                    title: r.original,
                })
                .collect();

        let to_insert: Vec<NewTag> = fresh_tags
            .into_iter()
            .filter(|t| !existing_map.contains(&t.title))
            .collect();

        if to_insert.is_empty() {
            return Ok(existing_by_titles);
        }

        NewTag::insert_batch(&mut self.rb.clone(), &to_insert, to_insert.len() as u64).await?;

        let all_tags = RbatisPostService::get_tags_by_titles(&self.rb, &search_titles).await?;
        Ok(all_tags)
    }

    async fn merge_post_tags(&self, post_id: &u64, tags: Vec<Tag>) -> DResult<()> {
        let new_tags_map: HashSet<u64> = tags.into_iter().fold(HashSet::new(), |mut set, tag| {
            set.insert(tag.id);
            set
        });

        let existing_tags_map = PostTag::select_all_by_post_id(&mut self.rb.clone(), post_id)
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
