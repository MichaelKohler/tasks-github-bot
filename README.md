# tasks-github-bot

This application searches for Bugzilla bugs and GitHub issues assigned to a given user. For all of these (if not yet existing)
it will create a new issue in a specified GitHub project.

## Run

```
GITHUB_TOKEN=<yourGitHubToken> BUGZILLA_EMAIL=<assigneeBugzillaEmail> GITHUB_OWNER=<githubRepoOwner> GITHUB_REPO=<githubRepo> cargo run
```