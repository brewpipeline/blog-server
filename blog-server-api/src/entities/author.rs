use blog_server_services::traits::author_service::Author as ServiceAuthor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShortAuthor {
    pub slug: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl Into<ShortAuthor> for ServiceAuthor {
    fn into(self) -> ShortAuthor {
        ShortAuthor {
            slug: self.base.slug,
            first_name: self.base.first_name,
            last_name: self.base.last_name,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub slug: String,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub registered_at: i64,
    pub status: Option<String>,
}

impl Into<Author> for ServiceAuthor {
    fn into(self) -> Author {
        Author {
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
