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
            id: self.id,
            slug: self.base.slug,
            first_name: self.base.first_name,
            middle_name: self.base.middle_name,
            last_name: self.base.last_name,
            mobile: self.base.mobile,
            email: self.base.email,
            registered_at: self.base.registered_at,
            status: self.base.status,
        }
    }
}
