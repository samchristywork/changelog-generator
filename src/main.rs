use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn walk_git_history(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let repo = git2::Repository::discover(path)?;

    let mut file = File::create("changelog").unwrap();

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        let message = String::from_utf8_lossy(
            commit
                .summary_bytes()
                .unwrap_or_else(|| commit.message_bytes()),
        );
        let body = String::from_utf8_lossy(
            commit
                .body_bytes()
                .unwrap_or_else(|| "<Empty Body>".as_bytes()),
        );

        println!("ID: {}", commit.id());
        println!("Message: {}", message);

        body.split('\n').for_each(|i| {
            if i.contains("Package:") {
                let mut meta = i.split(|c| c == ':' || c == '@');
                let _ = meta.next().unwrap();
                let package: String = meta
                    .next()
                    .unwrap_or_else(|| "<Empty Package>")
                    .split_whitespace()
                    .collect();
                let version: String = meta
                    .next()
                    .unwrap_or_else(|| "<Empty Version>")
                    .split_whitespace()
                    .collect();
                println!("Package: {}", package);
                println!("Version: {}", version);
                println!("\n\n");
            }
        });

        writeln!(file, "{}", message).unwrap();
    }
    Ok(())
}

fn main() {
    walk_git_history(PathBuf::from(r"/home/sam/git/changelog-generator")).unwrap();
}
