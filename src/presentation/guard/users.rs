use actix_web::{guard::Guard, web};

use crate::infrastructure::repositories::mysql_users_repository::MysqlUsersRepository;

pub struct UserGuard;

impl Guard for UserGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        let users = ctx.app_data::<web::Data<MysqlUsersRepository>>().unwrap();
        if let Some(token) = ctx.head().headers().get("token") {
            users.validate(token.to_str().unwrap_or_default()).is_ok()
        } else {
            false
        }
    }
}
