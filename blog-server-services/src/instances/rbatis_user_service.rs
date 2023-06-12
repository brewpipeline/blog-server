use crate::traits::user_service::{User, UserService};
use rbatis::rbatis::RBatis;
use screw_components::dyn_result::DResult;

pub fn create_rbatis_user_service(rb: RBatis) -> Box<dyn UserService> {
    Box::new(RbatisUserService { rb })
}

crud!(User {});

struct RbatisUserService {
    rb: RBatis,
}

#[async_trait]
impl UserService for RbatisUserService {
    async fn get_user(&self, username: &String) -> DResult<Option<User>> {
        Ok(
            User::select_by_column(&mut self.rb.clone(), "username", username)
                .await?
                .pop(),
        )
    }
    async fn create_user(&self, user: &User) -> DResult<()> {
        let _ = User::insert(&mut self.rb.clone(), user).await?;
        Ok(())
    }
}
