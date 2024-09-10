use crate::domain::primitive::OrganizationName;
use crate::port::display::DisplayPort;
use crate::port::github::GitHubPort;

pub async fn output_one_organization_repository<T: GitHubPort, U: DisplayPort>(
    github_port: T,
    display_port: U,
    organization_name: &OrganizationName,
) {
    let repository_name = github_port
        .get_organization_repositories(organization_name)
        .await;

    match repository_name {
        Ok(ref repository_name) => {
            display_port
                .print_repository_with_organization(organization_name, repository_name)
                .await;
        }
        Err(_) => {
            display_port.print_error("failed to get repository").await;
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use mockall::predicate;
    use predicate::eq;

    use crate::port::display::MockDisplayPort;
    use crate::port::github::MockGitHubPort;
    use crate::use_case::repository::output_one_organization_repository;

    #[tokio::test]
    async fn output_one_organization_repository_name() {
        let mut stub_github_port = MockGitHubPort::new();
        stub_github_port
            .expect_get_organization_repositories()
            .with(eq(String::from("rust-lang")))
            .returning(|_| {
                Ok(vec![
                    String::from("rust"),
                    String::from("rustlings"),
                    String::from("cargo"),
                ])
            });

        let mut mock_display_port = MockDisplayPort::new();
        mock_display_port
            .expect_print_repository_with_organization()
            .with(
                eq(String::from("rust-lang")),
                eq(vec![
                    String::from("rust"),
                    String::from("rustlings"),
                    String::from("cargo"),
                ]),
            )
            .times(1)
            .return_const(());

        output_one_organization_repository(
            stub_github_port,
            mock_display_port,
            &String::from("rust-lang"),
        )
        .await;
    }

    #[tokio::test]
    async fn output_error_message_failed_to_get_organization_repository_name() {
        let mut stub_github_port = MockGitHubPort::new();
        stub_github_port
            .expect_get_organization_repositories()
            .with(eq(String::from("rust-lang")))
            .returning(|_| Err(anyhow!("failed")));

        let mut mock_display_port = MockDisplayPort::new();
        mock_display_port
            .expect_print_error()
            .with(eq("failed to get repository"))
            .times(1)
            .return_const(());

        output_one_organization_repository(
            stub_github_port,
            mock_display_port,
            &String::from("rust-lang"),
        )
        .await;
    }
}
