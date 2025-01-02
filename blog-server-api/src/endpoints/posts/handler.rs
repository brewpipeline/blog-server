use std::sync::Arc;

use crate::utils::auth;
use blog_generic::entities::{PostsContainer, PublishedType, TotalOffsetLimitContainer};
use blog_server_services::traits::author_service::Author;
use blog_server_services::traits::entity_post_service::EntityPostService;
use blog_server_services::traits::post_service::{PostService, PostsRequest, PostsResponse};
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

    let posts_result = match handler_type {
        HandlerType::Published => match filter {
            Some(Filter::SearchQuery(search_query)) => {
                post_service
                    .posts(PostsRequest {
                        query: Some(&search_query),
                        author_id: None,
                        tag_id: None,
                        published_type: Some(&PublishedType::Published),
                        offset: &offset,
                        limit: &limit,
                    })
                    .await
            }
            Some(Filter::AuthorId(author_id)) => {
                post_service
                    .posts(PostsRequest {
                        query: None,
                        author_id: Some(&author_id),
                        tag_id: None,
                        published_type: Some(&PublishedType::Published),
                        offset: &offset,
                        limit: &limit,
                    })
                    .await
            }
            Some(Filter::TagId(tag_id)) => {
                post_service
                    .posts(PostsRequest {
                        query: None,
                        author_id: None,
                        tag_id: Some(&tag_id),
                        published_type: Some(&PublishedType::Published),
                        offset: &offset,
                        limit: &limit,
                    })
                    .await
            }
            None => {
                post_service
                    .posts(PostsRequest {
                        query: None,
                        author_id: None,
                        tag_id: None,
                        published_type: Some(&PublishedType::Published),
                        offset: &offset,
                        limit: &limit,
                    })
                    .await
            }
        },
        HandlerType::Unpublished { auth_author_future } => {
            if let Some(author) = auth_author_future.await.ok() {
                match filter {
                    Some(Filter::SearchQuery(_)) => unimplemented!(),
                    Some(Filter::AuthorId(author_id)) => {
                        if author.base.editor == 1 || author_id == author.id {
                            post_service
                                .posts(PostsRequest {
                                    query: None,
                                    author_id: Some(&author_id),
                                    tag_id: None,
                                    published_type: Some(&PublishedType::Unpublished),
                                    offset: &offset,
                                    limit: &limit,
                                })
                                .await
                        } else {
                            Ok(PostsResponse {
                                total_count: 0,
                                posts: vec![],
                            })
                        }
                    }
                    Some(Filter::TagId(_)) => unimplemented!(),
                    None => {
                        if author.base.editor == 1 {
                            post_service
                                .posts(PostsRequest {
                                    query: None,
                                    author_id: None,
                                    tag_id: None,
                                    published_type: Some(&PublishedType::Unpublished),
                                    offset: &offset,
                                    limit: &limit,
                                })
                                .await
                        } else {
                            Ok(PostsResponse {
                                total_count: 0,
                                posts: vec![],
                            })
                        }
                    }
                }
            } else {
                Ok(PostsResponse {
                    total_count: 0,
                    posts: vec![],
                })
            }
        }
    };

    let posts_response = posts_result.map_err(|e| DatabaseError {
        reason: e.to_string(),
    })?;

    let posts_entities = entity_post_service
        .posts_entities(posts_response.posts)
        .await
        .map_err(|e| DatabaseError {
            reason: e.to_string(),
        })?;

    Ok(PostsContainer {
        posts: posts_entities,
        base: TotalOffsetLimitContainer {
            total: posts_response.total_count,
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
        filter: None,
        offset: Some(offset),
        limit: Some(limit),
        post_service,
        entity_post_service,
    },))
    .await
    .ok()
    .map(|s| s.container)
}
