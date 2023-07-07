use blog_server_services::traits::author_service::Author as ServiceAuthor;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    id: i64,
    slug: String,
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    mobile: Option<String>,
    email: Option<String>,
    registered_at: i64,
    status: Option<String>,
}

impl Into<Author> for ServiceAuthor {
    fn into(self) -> Author {
        Author {
            id: self.id.expect("should convert only full authors"),
            slug: self.slug,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            mobile: self.mobile,
            email: self.email,
            registered_at: self.registered_at,
            status: self.status,
        }
    }
}
