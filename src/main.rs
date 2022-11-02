use std::path::PathBuf;

fn walk_git_history(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git2::Repository::discover(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        let message = commit
            .summary_bytes()
            .unwrap_or_else(|| commit.message_bytes());
        let body = commit
            .body_bytes()
            .unwrap_or_else(|| "<Empty Body>".as_bytes());

        println!("{}", commit.id());
        println!("{}", String::from_utf8_lossy(message));
        println!("{}\n\n", String::from_utf8_lossy(body));
    }
    Ok(())
}

fn main() {
    walk_git_history(PathBuf::from(r"/home/sam/git/changelog-generator")).unwrap();
}
