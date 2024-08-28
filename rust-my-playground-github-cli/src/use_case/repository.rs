use crate::port::display::DisplayPort;
use crate::port::github::GitHubPort;

pub async fn output_one_organization_repository<T: GitHubPort, U: DisplayPort>(
    github_port: T,
    display_port: U,
    organization_name: &str,
) {
    let repository_name = github_port
        .get_one_organization_repository(organization_name)
        .await;

    display_port
        .print_repository_with_organization(organization_name, &repository_name)
        .await;
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use predicate::eq;

    use crate::port::display::MockDisplayPort;
    use crate::port::github::MockGitHubPort;
    use crate::use_case::repository::output_one_organization_repository;

    #[tokio::test]
    async fn give_exist_organization_then_output_one_repository() {
        let mut mock_github_port = MockGitHubPort::new();
        mock_github_port
            .expect_get_one_organization_repository()
            .with(eq("rust-lang"))
            .times(1)
            .return_const("cargo");

        let mut mock_display_port = MockDisplayPort::new();
        mock_display_port
            .expect_print_repository_with_organization()
            .with(eq("rust-lang"), eq("cargo"))
            .times(1)
            .return_const(());

        output_one_organization_repository(mock_github_port, mock_display_port, "rust-lang").await;
    }
}