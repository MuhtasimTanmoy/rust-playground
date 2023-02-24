#[derive(Debug, Default)]
pub struct UserStruct {}

trait UserSvc {
    fn name(&self) -> &'static str;
}

#[tonic::async_trait]
impl UserSvc for UserStruct {
    fn name(&self) -> &'static str {
        "test"
    }
}