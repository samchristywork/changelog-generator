use git2::Repository;

fn main() {
    let repo = match Repository::open("/home/sam/git/changelog-generator") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    println!("{:?}", repo.state());
    let oid = match git2::Oid::from_str("77b0c17a291c568d31534b3cb37732c2f3c54f7e") {
        Ok(oid) => oid,
        Err(e) => panic!("asdf {}", e),
    };
    println!("{:?}",repo.find_commit(oid));
}
