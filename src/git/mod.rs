use git2::{Oid, Repository, Revwalk};

pub fn get_commit_messages_from(from_commit_hash: &str) -> Vec<String> {
    let mut commit_messages = vec![];

    let repository = get_repository();
    let mut revwalk = get_revwalk(&repository, get_oid(from_commit_hash));

    loop {
        match revwalk.next() {
            Some(Ok(oid)) => match get_commit_message(&repository, oid) {
                Some(commit_message) => {
                    trace!("Found commit '{}'s message '{:?}'.", oid, commit_message);
                    commit_messages.push(commit_message);
                }
                None => {
                    warn!("Commit '{}' has no message.", oid);
                }
            },
            Some(Err(error)) => {
                error!("Revwalk threw error while walking the commits.");
                error!("{:?}", error);
                std::process::exit(1);
            }
            None => break,
        }
    }

    debug!(
        "Found '{}' commit message between HEAD and --from-commit.",
        commit_messages.len()
    );
    commit_messages.reverse();
    commit_messages
}

fn get_revwalk(repository: &Repository, from_commit_hash: Oid) -> Revwalk {
    match repository.revwalk() {
        Ok(mut revwalk) => {
            match revwalk.push_head() {
                Ok(_result) => {}
                Err(_error) => {
                    error!("Unable to push HEAD onto the revwalk.");
                    std::process::exit(1);
                }
            }

            match revwalk.hide(from_commit_hash) {
                Ok(_result) => {}
                Err(_error) => {
                    error!(
                        "Can not find --from-commit '{}' on the revwalk.",
                        from_commit_hash
                    );
                    std::process::exit(1);
                }
            }

            revwalk
        }
        Err(error) => {
            error!("Unable to get revwalk from local repository.");
            error!("{:?}", error);
            std::process::exit(1);
        }
    }
}

fn get_commit_message(repository: &Repository, oid: Oid) -> Option<String> {
    match repository.find_commit(oid) {
        Ok(commit) => commit.message().map(|m| m.to_string()),

        Err(_error) => {
            error!("Can not find commit '{}' in current repository.", oid);
            std::process::exit(1);
        }
    }
}

fn get_repository() -> Repository {
    match Repository::open_from_env() {
        Ok(repository) => repository,
        Err(error) => {
            error!("Unable to open the Git repository.");
            error!("{:?}", error);
            std::process::exit(1);
        }
    }
}

fn get_oid(oid_string: &str) -> Oid {
    match Oid::from_str(oid_string) {
        Ok(oid) => oid,
        Err(_error) => {
            error!("'{}' is not a valid commit id.", oid_string);
            std::process::exit(1);
        }
    }
}
