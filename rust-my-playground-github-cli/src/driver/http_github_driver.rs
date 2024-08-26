use async_trait::async_trait;

use crate::port::github::GitHubPort;

pub struct HttpGithubDriver {
    auth_token: String,
}

impl HttpGithubDriver {
    pub fn new(auth_token: String) -> HttpGithubDriver {
        HttpGithubDriver { auth_token }
    }
}

#[async_trait]
impl GitHubPort for HttpGithubDriver {
    async fn get_one_organization_repository(&self, _organization_name: &str) -> String {
        todo!()
    }
}
