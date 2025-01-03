use std::sync::Arc;

use crate::utils::auth;
use blog_generic::entities::{PostsContainer, PublishType, TotalOffsetLimitContainer};
use blog_server_services::traits::author_service::Author;
use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::post_service::{PostService, PostsQuery, PostsQueryAnswer};
use screw_components::dyn_fn::DFuture;

use super::request_content::{PostsRequestContentFilter as Filter, *};
use super::response_content_failure::PostsResponseContentFailure;
use super::response_content_failure::PostsResponseContentFailure::*;
use super::response_content_success::PostsResponseContentSuccess;

pub async fn http_handler(
    (posts_request_content,): (PostsRequestContent,),
) -> Result<PostsResponseContentSuccess, PostsResponseContentFailure> {
    handler(posts_request_content, HandlerType::Published).await
}

pub async fn http_handler_unpublished(
    (UnpublishedPostsRequestContent {
        base: posts_request_content,
        auth_author_future,
    },): (UnpublishedPostsRequestContent,),
) -> Result<PostsResponseContentSuccess, PostsResponseContentFailure> {
    handler(
        posts_request_content,
        HandlerType::AuthRequired {
            inner_type: HandlerTypeAuthRequired::Unpublished,
            auth_author_future,
        },
    )
    .await
}

pub async fn http_handler_hidden(
    (UnpublishedPostsRequestContent {
        base: posts_request_content,
        auth_author_future,
    },): (UnpublishedPostsRequestContent,),
) -> Result<PostsResponseContentSuccess, PostsResponseContentFailure> {
    handler(
        posts_request_content,
        HandlerType::AuthRequired {
            inner_type: HandlerTypeAuthRequired::Hidden,
            auth_author_future,
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
        auth_author_future: DFuture<Result<Author, auth::Error>>,
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
        HandlerType::AuthRequired {
            inner_type,
            auth_author_future,
        } => {
            let author = auth_author_future.await.map_err(|e| Unauthorized {
                reason: e.to_string(),
            })?;
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

pub async fn direct_handler(
    offset: u64,
    limit: u64,
    post_service: Arc<Box<dyn PostService>>,
    entity_post_service: Arc<Box<dyn EntityPostService>>,
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
