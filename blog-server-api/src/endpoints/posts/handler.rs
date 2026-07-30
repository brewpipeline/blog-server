use std::sync::Arc;

use blog_generic::entities::{PostsContainer, PublishType, TotalOffsetLimitContainer};
use blog_server_services::traits::author_service::Author;
use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::post_service::{PostService, PostsQuery, PostsQueryAnswer};

use super::request_content::*;
use super::response_content_failure::PostsResponseContentFailure;
use super::response_content_failure::PostsResponseContentFailure::*;
use super::response_content_success::PostsResponseContentSuccess;

pub async fn http_handler(
    (posts_request_content,): (PostsRequestContent,),
) -> Result<PostsResponseContentSuccess, PostsResponseContentFailure> {
    handler(posts_request_content, HandlerType::Published).await
}

pub async fn http_handler_unpublished(
    (author, posts_request_content): (Author, PostsRequestContent),
) -> Result<PostsResponseContentSuccess, PostsResponseContentFailure> {
    handler(
        posts_request_content,
        HandlerType::AuthRequired {
            inner_type: HandlerTypeAuthRequired::Unpublished,
            author,
        },
    )
    .await
}

pub async fn http_handler_hidden(
    (author, posts_request_content): (Author, PostsRequestContent),
) -> Result<PostsResponseContentSuccess, PostsResponseContentFailure> {
    handler(
        posts_request_content,
        HandlerType::AuthRequired {
            inner_type: HandlerTypeAuthRequired::Hidden,
            author,
        },
    )
    .await
}

enum HandlerTypeAuthRequired {
    Unpublished,
    Hidden,
}

enum HandlerType {
    Published,
    AuthRequired {
        inner_type: HandlerTypeAuthRequired,
        author: Author,
    },
}

async fn handler(
    PostsRequestContent {
        filter,
        offset,
        limit,
        post_service,
        entity_post_service,
    }: PostsRequestContent,
    handler_type: HandlerType,
) -> Result<PostsResponseContentSuccess, PostsResponseContentFailure> {
    let offset = offset.unwrap_or(0).max(0);
    let limit = limit.unwrap_or(50).max(0).min(50);

    let publish_type = match handler_type {
        HandlerType::Published => PublishType::Published,
        HandlerType::AuthRequired { inner_type, author } => {
            if !(filter.author_id == Some(author.id) || author.base.editor == 1) {
                return Err(Forbidden);
            }
            match inner_type {
                HandlerTypeAuthRequired::Unpublished => PublishType::Unpublished,
                HandlerTypeAuthRequired::Hidden => PublishType::Hidden,
            }
        }
    };

    let posts_query = PostsQuery::offset_and_limit(&offset, &limit)
        .publish_type(Some(&publish_type))
        .search_query(Option::from(&filter.search_query))
        .author_id(Option::from(&filter.author_id))
        .tag_id(Option::from(&filter.tag_id));

    let PostsQueryAnswer { total_count, posts } =
        post_service
            .posts(posts_query)
            .await
            .map_err(|e| DatabaseError {
                reason: e.to_string(),
            })?;

    let posts_entities = entity_post_service
        .posts_entities(posts)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(PostsContainer {
        posts: posts_entities,
        base: TotalOffsetLimitContainer {
            total: total_count,
            offset,
            limit,
        },
    }
    .into())
}

#[cfg_attr(not(feature = "ssr"), allow(dead_code))]
pub async fn direct_handler(
    offset: u64,
    limit: u64,
    post_service: Arc<dyn PostService>,
    entity_post_service: Arc<dyn EntityPostService>,
) -> Option<PostsContainer> {
    http_handler((PostsRequestContent {
        filter: PostsRequestContentFilter {
            search_query: None,
            author_id: None,
            tag_id: None,
        },
        offset: Some(offset),
        limit: Some(limit),
        post_service,
        entity_post_service,
    },))
    .await
    .ok()
    .map(|s| s.container)
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use blog_generic::entities::Post as EPost;
    use blog_server_services::traits::{
        author_service::{Author as SAuthor, BaseAuthor},
        entity_post_service::EntityPostService,
        post_service::{Post, PostService},
    };
    use screw_components::dyn_result::DResult;

    use super::super::request_content::PostsRequestContentFilter as Filter;

    enum PostBehavior {
        Success(u64),
        Error,
    }

    struct MockPostService {
        behavior: PostBehavior,
    }

    #[async_trait]
    impl PostService for MockPostService {
        async fn posts<'q, 'a, 't, 'p, 'o, 'l>(
            &self,
            _request: PostsQuery<'q, 'a, 't, 'p, 'o, 'l>,
        ) -> DResult<PostsQueryAnswer> {
            match self.behavior {
                PostBehavior::Success(total) => Ok(PostsQueryAnswer {
                    total_count: total,
                    posts: vec![],
                }),
                PostBehavior::Error => Err("db error".into()),
            }
        }

        async fn post_by_id(&self, _id: &u64) -> DResult<Option<Post>> {
            unimplemented!()
        }

        async fn create_post(
            &self,
            _post: &blog_server_services::traits::post_service::BasePost,
        ) -> DResult<u64> {
            unimplemented!()
        }

        async fn update_post_by_id(
            &self,
            _id: &u64,
            _post: &blog_server_services::traits::post_service::BasePost,
            _update_created_at: &bool,
        ) -> DResult<()> {
            unimplemented!()
        }

        async fn delete_post_by_id(&self, _id: &u64) -> DResult<()> {
            unimplemented!()
        }

        async fn tag_by_id(
            &self,
            _id: &u64,
        ) -> DResult<Option<blog_server_services::traits::post_service::Tag>> {
            unimplemented!()
        }

        async fn random_recommended_post(&self, _post_id: &u64) -> DResult<Option<Post>> {
            unimplemented!()
        }
        async fn set_post_recommended_by_id(&self, _id: &u64, _recommended: &u8) -> DResult<()> {
            unimplemented!()
        }

        async fn create_tags(
            &self,
            _tag_titles: Vec<String>,
        ) -> DResult<Vec<blog_server_services::traits::post_service::Tag>> {
            unimplemented!()
        }

        async fn merge_post_tags(
            &self,
            _post_id: &u64,
            _tags: Vec<blog_server_services::traits::post_service::Tag>,
        ) -> DResult<()> {
            unimplemented!()
        }
    }

    enum EntityBehavior {
        Success(Vec<EPost>),
        Error,
    }

    struct MockEntityPostService {
        behavior: EntityBehavior,
    }

    #[async_trait]
    impl EntityPostService for MockEntityPostService {
        async fn posts_entities(&self, _posts: Vec<Post>) -> DResult<Vec<EPost>> {
            match &self.behavior {
                EntityBehavior::Success(posts) => Ok(posts.clone()),
                EntityBehavior::Error => Err("db error".into()),
            }
        }
    }

    fn empty_request(
        post_service: Arc<dyn PostService>,
        entity_post_service: Arc<dyn EntityPostService>,
    ) -> PostsRequestContent {
        PostsRequestContent {
            filter: Filter {
                search_query: None,
                author_id: None,
                tag_id: None,
            },
            offset: None,
            limit: None,
            post_service,
            entity_post_service,
        }
    }

    fn sample_author(editor: u8, id: u64) -> SAuthor {
        SAuthor {
            id,
            base: BaseAuthor {
                slug: "john".into(),
                first_name: None,
                middle_name: None,
                last_name: None,
                mobile: None,
                email: None,
                password_hash: None,
                registered_at: 0,
                status: None,
                image_url: None,
                editor,
                blocked: 0,
                yandex_id: None,
                telegram_id: None,
                notification_subscribed: None,
                override_social_data: 0,
            },
        }
    }

    #[tokio::test]
    async fn forbidden_when_author_mismatch() {
        let post_service = Arc::new(MockPostService {
            behavior: PostBehavior::Success(0),
        });
        let entity_post_service = Arc::new(MockEntityPostService {
            behavior: EntityBehavior::Success(vec![]),
        });
        let request = PostsRequestContent {
            filter: Filter {
                search_query: None,
                author_id: Some(2),
                tag_id: None,
            },
            offset: None,
            limit: None,
            post_service,
            entity_post_service,
        };
        let result = http_handler_unpublished((sample_author(0, 1), request)).await;
        assert!(matches!(result, Err(Forbidden)));
    }

    #[tokio::test]
    async fn editor_may_read_other_authors_unpublished() {
        let post_service = Arc::new(MockPostService {
            behavior: PostBehavior::Success(0),
        });
        let entity_post_service = Arc::new(MockEntityPostService {
            behavior: EntityBehavior::Success(vec![]),
        });
        let request = PostsRequestContent {
            filter: Filter {
                search_query: None,
                author_id: Some(2),
                tag_id: None,
            },
            offset: None,
            limit: None,
            post_service,
            entity_post_service,
        };
        let result = http_handler_unpublished((sample_author(1, 1), request)).await;
        assert!(matches!(result, Ok(_)));
    }

    #[tokio::test]
    async fn database_error_from_post_service() {
        let post_service = Arc::new(MockPostService {
            behavior: PostBehavior::Error,
        });
        let entity_post_service = Arc::new(MockEntityPostService {
            behavior: EntityBehavior::Success(vec![]),
        });
        let result = http_handler((empty_request(post_service, entity_post_service),)).await;
        assert!(matches!(result, Err(DatabaseError { .. })));
    }

    #[tokio::test]
    async fn database_error_from_entity_service() {
        let post_service = Arc::new(MockPostService {
            behavior: PostBehavior::Success(0),
        });
        let entity_post_service = Arc::new(MockEntityPostService {
            behavior: EntityBehavior::Error,
        });
        let result = http_handler((empty_request(post_service, entity_post_service),)).await;
        assert!(matches!(result, Err(DatabaseError { .. })));
    }

    #[tokio::test]
    async fn success_returns_posts() {
        let post_service = Arc::new(MockPostService {
            behavior: PostBehavior::Success(0),
        });
        let entity_post_service = Arc::new(MockEntityPostService {
            behavior: EntityBehavior::Success(vec![]),
        });
        let result = http_handler((empty_request(post_service, entity_post_service),)).await;
        assert!(matches!(result, Ok(_)));
    }
}
