use crate::traits::post_service::{BasePost, Post, PostService, PostsQuery, PostsQueryAnswer, Tag};
use crate::utils::{string_filter, transliteration};
use blog_generic::entities::PublishType;
use rbatis::executor::RBatisTxExecutorGuard;
use rbatis::{rbatis::RBatis, rbdc::db::ExecResult};
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

impl From<TagDto> for Tag {
    fn from(value: TagDto) -> Self {
        Tag {
            id: value.id,
            title: value.title,
            slug: value.slug,
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
        rb: &mut RBatisTxExecutorGuard,
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
            )
            if lang != '':
                AND (post.lang = #{lang} OR post.lang IS NULL)
        ORDER BY random() \
        LIMIT 1 \
    "
    )]
    async fn random_recommended_post(
        rb: &RBatis,
        post_id: &u64,
        lang: &str,
    ) -> rbatis::Result<Option<Post>> {
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
        (author_id,title,slug,summary,publish_type,created_at,content,plain_text_content,image_url,lang)
        VALUES
        (#{post.author_id},#{post.title},#{post.slug},#{post.summary},#{post.publish_type},to_timestamp(#{post.created_at}),#{post.content},#{post.plain_text_content},#{post.image_url},#{post.lang})
        RETURNING id
    "
    )]
    async fn insert_new_post(
        rb: &mut RBatisTxExecutorGuard,
        post: &BasePost,
    ) -> rbatis::Result<u64> {
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
            image_url = #{post_data.image_url}, \
            lang = #{post_data.lang} \
        WHERE id = #{post_id} \
        RETURNING id
    "
    )]
    async fn update_post_by_id(
        rb: &mut RBatisTxExecutorGuard,
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
    async fn get_tags_by_titles(
        rb: &mut RBatisTxExecutorGuard,
        titles: &Vec<String>,
    ) -> rbatis::Result<Vec<Tag>> {
        impled!()
    }
    #[py_sql(
        "
        INSERT INTO tag (title, slug) \
        VALUES
        trim ',': for _,tag in tags:
            (#{tag.title}, #{tag.slug}),
        ON CONFLICT (title) DO NOTHING
    "
    )]
    async fn insert_tags_ignoring_conflicts(
        rb: &mut RBatisTxExecutorGuard,
        tags: &Vec<NewTag>,
    ) -> rbatis::Result<ExecResult> {
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

    #[py_sql(
        "
        SELECT \
            post.*
        if search_query != null:
            , ts_rank_cd(textsearch.vector, query) AS rank
        FROM post
        if tag_id != null:
            JOIN post_tag ON post.id = post_tag.post_id
        if search_query != null:
            , plainto_tsquery('${ts_config}'::regconfig, LOWER(#{search_query})) query \
            , LATERAL (SELECT setweight(to_tsvector('${ts_config}'::regconfig, LOWER(post.title)), 'A') || setweight(to_tsvector('${ts_config}'::regconfig, LOWER(post.summary)), 'B') || setweight(to_tsvector('${ts_config}'::regconfig, LOWER(COALESCE(post.plain_text_content, ''))), 'C') AS vector) textsearch
        where:
            if search_query != null:
                and textsearch.vector @@ query
            if author_id != null:
                and post.author_id = #{author_id}
            if tag_id != null:
                and post_tag.tag_id = #{tag_id}
            if publish_type != null:
                and post.publish_type = #{publish_type}
            if lang != '':
                and (post.lang = #{lang} OR post.lang IS NULL)
        ORDER BY
        if search_query != null:
            rank DESC,
        post.id DESC \
        LIMIT #{limit} OFFSET #{offset}
    "
    )]
    async fn select_posts(
        rb: &RBatis,
        search_query: Option<&String>,
        author_id: Option<&u64>,
        tag_id: Option<&u64>,
        publish_type: Option<&PublishType>,
        lang: &str,
        ts_config: &str,
        offset: &u64,
        limit: &u64,
    ) -> rbatis::Result<Vec<Post>> {
        impled!()
    }

    #[py_sql(
        "
        SELECT COUNT(1) \
        FROM post
        if tag_id != null:
            JOIN post_tag ON post.id = post_tag.post_id
        if search_query != null:
            , plainto_tsquery('${ts_config}'::regconfig, LOWER(#{search_query})) query \
            , LATERAL (SELECT setweight(to_tsvector('${ts_config}'::regconfig, LOWER(post.title)), 'A') || setweight(to_tsvector('${ts_config}'::regconfig, LOWER(post.summary)), 'B') || setweight(to_tsvector('${ts_config}'::regconfig, LOWER(COALESCE(post.plain_text_content, ''))), 'C') AS vector) textsearch
        where:
            if search_query != null:
                and textsearch.vector @@ query
            if author_id != null:
                and post.author_id = #{author_id}
            if tag_id != null:
                and post_tag.tag_id = #{tag_id}
            if publish_type != null:
                and post.publish_type = #{publish_type}
            if lang != '':
                and (post.lang = #{lang} OR post.lang IS NULL)
    "
    )]
    async fn count_posts(
        rb: &RBatis,
        search_query: Option<&String>,
        author_id: Option<&u64>,
        tag_id: Option<&u64>,
        publish_type: Option<&PublishType>,
        lang: &str,
        ts_config: &str,
    ) -> rbatis::Result<u64> {
        impled!()
    }

    async fn begin(&self) -> DResult<RBatisTxExecutorGuard> {
        let tx = self.rb.acquire_begin().await?;
        Ok(tx.defer_async(|tx| async move {
            if !tx.done() {
                let _ = tx.rollback().await;
            }
        }))
    }

    async fn create_tags(
        tx: &mut RBatisTxExecutorGuard,
        tag_titles: Vec<String>,
    ) -> DResult<Vec<Tag>> {
        if tag_titles.is_empty() {
            return Ok(vec![]);
        }
        let tag_titles: Vec<String> = tag_titles
            .into_iter()
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();
        let search_titles = tag_titles.clone();

        let to_insert: Vec<NewTag> =
            transliteration::ru_to_latin(tag_titles, transliteration::TranslitOption::ToLowerCase)
                .into_iter()
                .map(|r| NewTag {
                    slug: string_filter::remove_non_latin_or_number_chars(&r.transliterated),
                    title: r.original,
                })
                .collect();

        RbatisPostService::insert_tags_ignoring_conflicts(tx, &to_insert).await?;

        let all_tags = RbatisPostService::get_tags_by_titles(tx, &search_titles).await?;
        Ok(all_tags)
    }

    async fn merge_post_tags(
        tx: &mut RBatisTxExecutorGuard,
        post_id: &u64,
        tags: Vec<Tag>,
    ) -> DResult<()> {
        let new_tags_map: HashSet<u64> = tags.into_iter().fold(HashSet::new(), |mut set, tag| {
            set.insert(tag.id);
            set
        });

        let existing_tags_map = PostTag::select_all_by_post_id(&*tx, post_id)
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
            PostTag::insert_batch(&*tx, &to_insert, to_insert.len() as u64).await?;
        }
        if !to_delete.is_empty() {
            PostTag::delete_by_post_id_and_tag_ids(tx, *post_id, to_delete).await?;
        }

        Ok(())
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
    async fn posts<'q, 'a, 't, 'p, 'o, 'l>(
        &self,
        query: PostsQuery<'q, 'a, 't, 'p, 'o, 'l>,
    ) -> DResult<PostsQueryAnswer> {
        let lang = BasePost::current_lang().unwrap_or_default();
        let ts_config = BasePost::current_text_search_config();

        let (posts, total_count) = tokio::try_join!(
            RbatisPostService::select_posts(
                &self.rb,
                query.search_query,
                query.author_id,
                query.tag_id,
                query.publish_type,
                &lang,
                ts_config,
                query.offset,
                query.limit,
            ),
            RbatisPostService::count_posts(
                &self.rb,
                query.search_query,
                query.author_id,
                query.tag_id,
                query.publish_type,
                &lang,
                ts_config,
            ),
        )?;

        let posts_with_tags = self.saturate_posts_with_tags(posts).await?;

        Ok(PostsQueryAnswer {
            total_count,
            posts: posts_with_tags,
        })
    }

    async fn post_by_id(&self, id: &u64) -> DResult<Option<Post>> {
        let post_option = Post::single_by_id(&self.rb, id).await?;
        RbatisPostService::saturate_with_tags(&self, post_option).await
    }

    async fn create_post(&self, post: &BasePost, tag_titles: Vec<String>) -> DResult<u64> {
        let mut tx = self.begin().await?;
        let inserted_id = RbatisPostService::insert_new_post(&mut tx, post).await?;
        let tags = RbatisPostService::create_tags(&mut tx, tag_titles).await?;
        RbatisPostService::merge_post_tags(&mut tx, &inserted_id, tags).await?;
        tx.commit().await?;
        Ok(inserted_id)
    }

    async fn update_post_by_id(
        &self,
        id: &u64,
        post_data: &BasePost,
        update_created_at: &bool,
        tag_titles: Vec<String>,
    ) -> DResult<()> {
        let mut tx = self.begin().await?;
        RbatisPostService::update_post_by_id(&mut tx, id, post_data, update_created_at).await?;
        let tags = RbatisPostService::create_tags(&mut tx, tag_titles).await?;
        RbatisPostService::merge_post_tags(&mut tx, id, tags).await?;
        tx.commit().await?;
        Ok(())
    }

    async fn delete_post_by_id(&self, id: &u64) -> DResult<()> {
        let mut tx = self.begin().await?;
        PostTag::delete_by_post_id(&mut tx, id).await?;
        RbatisPostService::delete_post_by_id(&mut tx, id).await?;
        tx.commit().await?;
        Ok(())
    }

    async fn random_recommended_post(&self, post_id: &u64) -> DResult<Option<Post>> {
        let lang = BasePost::current_lang().unwrap_or_default();
        let post_option = Post::random_recommended_post(&self.rb, post_id, &lang).await?;
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
}
