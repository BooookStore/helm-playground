use crate::domain::primitive::OrganizationName;
use crate::port::display::DisplayPort;
use crate::port::github::GitHubPort;

pub async fn output_one_organization_repository<T: GitHubPort, U: DisplayPort>(
    github_port: T,
    display_port: U,
    organization_name: &OrganizationName,
) {
    let repository_names = github_port
        .get_organization_repositories(organization_name)
        .await;

    match repository_names {
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

    use crate::domain::primitive::{ContributorName, OrganizationName, RepositoryName};
    use crate::domain::repository::Repository;
    use crate::port::display::MockDisplayPort;
    use crate::port::github::MockGitHubPort;
    use crate::use_case::repository::output_one_organization_repository;

    #[tokio::test]
    async fn output_organization_repositories_with_contributors() {
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
        stub_github_port
            .expect_get_repository_contributors()
            .with(
                eq(OrganizationName::from("rust-lang")),
                eq(RepositoryName::from("rust")),
            )
            .returning(|_, _| {
                Ok(vec![
                    ContributorName::from("bob"),
                    ContributorName::from("alice"),
                ])
            });

        let mut mock_display_port = MockDisplayPort::new();
        mock_display_port
            .expect_print_repositories_with_contributors()
            .with(
                eq(OrganizationName::from("rust-lang")),
                eq(vec![
                    Repository::new(
                        RepositoryName::from("rust"),
                        vec![ContributorName::from("bob"), ContributorName::from("alice")],
                    ),
                    Repository::new(RepositoryName::from("rustlings"), vec![]),
                    Repository::new(RepositoryName::from("cargo"), vec![]),
                ]),
            )
            .return_const(())
            .times(1);

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
