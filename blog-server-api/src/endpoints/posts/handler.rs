use crate::utils::auth;
use blog_generic::entities::{PostsContainer, TotalOffsetLimitContainer};
use blog_server_services::traits::author_service::Author;
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
        HandlerType::Unpublished { auth_author_future },
    )
    .await
}

enum HandlerType {
    Published,
    Unpublished {
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

    let (posts_result, total_result) = match handler_type {
        HandlerType::Published => match filter {
            Some(Filter::SearchQuery(search_query)) => tokio::join!(
                post_service.posts_by_query(&search_query, &offset, &limit),
                post_service.posts_count_by_query(&search_query),
            ),
            Some(Filter::AuthorId(author_id)) => tokio::join!(
                post_service.posts_by_author_id(&author_id, &offset, &limit),
                post_service.posts_count_by_author_id(&author_id),
            ),
            Some(Filter::TagId(tag_id)) => tokio::join!(
                post_service.posts_by_tag_id(&tag_id, &offset, &limit),
                post_service.posts_count_by_tag_id(&tag_id),
            ),
            None => tokio::join!(
                post_service.posts(&offset, &limit),
                post_service.posts_count(),
            ),
        },
        HandlerType::Unpublished { auth_author_future } => {
            if let Some(author) = auth_author_future.await.ok() {
                match filter {
                    Some(Filter::SearchQuery(_)) => unimplemented!(),
                    Some(Filter::AuthorId(author_id)) => {
                        if author.base.editor == 1 || author_id == author.id {
                            tokio::join!(
                                post_service
                                    .unpublished_posts_by_author_id(&author_id, &offset, &limit),
                                post_service.unpublished_posts_count_by_author_id(&author_id),
                            )
                        } else {
                            (Ok(vec![]), Ok(0))
                        }
                    }
                    Some(Filter::TagId(_)) => unimplemented!(),
                    None => {
                        if author.base.editor == 1 {
                            tokio::join!(
                                post_service.unpublished_posts(&offset, &limit),
                                post_service.unpublished_posts_count(),
                            )
                        } else {
                            (Ok(vec![]), Ok(0))
                        }
                    }
                }
            } else {
                (Ok(vec![]), Ok(0))
            }
        }
    };

    let posts = posts_result.map_err(|e| DatabaseError {
        reason: e.to_string(),
    })?;

    let total = total_result.map_err(|e| DatabaseError {
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
            total,
            offset,
            limit,
        },
    }
    .into())
}
