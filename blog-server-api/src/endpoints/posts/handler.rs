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

pub async fn http_handler_hidden(
    (UnpublishedPostsRequestContent {
        base: posts_request_content,
        auth_author_future,
    },): (UnpublishedPostsRequestContent,),
) -> Result<PostsResponseContentSuccess, PostsResponseContentFailure> {
    handler(
        posts_request_content,
        HandlerType::Hidden { auth_author_future },
    )
    .await
}

enum HandlerType {
    Published,
    Unpublished {
        auth_author_future: DFuture<Result<Author, auth::Error>>,
    },
    Hidden {
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
    let base_posts_request = PostsRequest::offset_and_limit(&offset, &limit);

    let posts_result = match handler_type {
        HandlerType::Published => match filter {
            Some(Filter::SearchQuery(search_query)) => {
                post_service
                    .posts(
                        base_posts_request
                            .published_type(Some(&PublishedType::Published))
                            .query(Some(&search_query)),
                    )
                    .await
            }
            Some(Filter::AuthorId(author_id)) => {
                post_service
                    .posts(
                        base_posts_request
                            .published_type(Some(&PublishedType::Published))
                            .author_id(Some(&author_id)),
                    )
                    .await
            }
            Some(Filter::TagId(tag_id)) => {
                post_service
                    .posts(
                        base_posts_request
                            .published_type(Some(&PublishedType::Published))
                            .tag_id(Some(&tag_id)),
                    )
                    .await
            }
            None => {
                post_service
                    .posts(
                        base_posts_request
                            .published_type(Some(&PublishedType::Published)),
                    )
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
                                .posts(
                                    base_posts_request
                                        .published_type(Some(&PublishedType::Unpublished))
                                        .author_id(Some(&author_id)),
                                )
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
                                .posts(
                                    base_posts_request
                                        .published_type(Some(&PublishedType::Unpublished)),
                                )
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
        HandlerType::Hidden { auth_author_future } => {
            let is_editor = auth_author_future
                .await
                .map(|a| a.base.editor == 1)
                .unwrap_or_default();
            if is_editor {
                match filter {
                    Some(Filter::SearchQuery(_)) => unimplemented!(),
                    Some(Filter::AuthorId(_)) => unimplemented!(),
                    Some(Filter::TagId(_)) => unimplemented!(),
                    None => {
                        post_service
                            .posts(
                                PostsRequest::offset_and_limit(&offset, &limit)
                                    .published_type(Some(&PublishedType::Hidden)),
                            )
                            .await
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
