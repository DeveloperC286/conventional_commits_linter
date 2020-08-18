use git2::{Oid, Repository, Revwalk};

pub fn get_commit_messages_till_head_from(
    from_commit_hash: Option<git2::Oid>,
    from_tag: Option<String>,
) -> Vec<String> {
    if from_commit_hash.is_some() && from_tag.is_some() {
        error!("Provide either the --from-tag or --from-commit-hash arguments not both.");
        std::process::exit(1);
    }

    if let Some(oid) = from_commit_hash {
        return get_commit_messages_till_head_from_oid(oid);
    }

    if let Some(tag_name) = from_tag {
        match get_tag_oid(&tag_name) {
            Some(oid) => {
                return get_commit_messages_till_head_from_oid(oid);
            }
            None => {
                error!("Could not find tag with the name '{}'.", tag_name);
                std::process::exit(1);
            }
        }
    }

    error!("Provide either the --from-tag or --from-commit-hash argument.");
    std::process::exit(1);
}

fn get_commit_messages_till_head_from_oid(from_commit_hash: Oid) -> Vec<String> {
    let mut commit_messages = vec![];

    let repository = get_repository();
    let mut revwalk = get_revwalk(&repository, from_commit_hash);

    loop {
        match revwalk.next() {
            Some(Ok(oid)) => match get_commit_message(&repository, oid) {
                Some(commit_message) => {
                    trace!("Found commit '{}'s message '{:?}'.", oid, commit_message);
                    commit_messages.push(commit_message);
                }
                None => {
                    warn!("Commit hash '{}' has no message.", oid);
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

    debug!("'{}' commit messages in the vector.", commit_messages.len());
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
                        "Can not find commit hash '{}' in the Git repository.",
                        from_commit_hash
                    );
                    std::process::exit(1);
                }
            }

            revwalk
        }
        Err(error) => {
            error!("Unable to get revwalk from local Git repository.");
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

fn get_tag_oid(matching: &str) -> Option<Oid> {
    let mut oid: Option<Oid> = None;
    let repository = get_repository();

    match repository.tag_foreach(|tag_oid: Oid, tag_name: &[u8]| -> bool {
        match std::str::from_utf8(tag_name) {
            Ok(tag_name) => {
                if tag_name.ends_with(matching) {
                    oid = Some(tag_oid);
                    return false;
                }
            }
            Err(error) => {
                error!("Unable to parse String from tag's name.");
                error!("{:?}", error);
                std::process::exit(1);
            }
        }

        true
    }) {
        Ok(_) => {}
        Err(error) => {
            error!("Unable to perform function on all tags.");
            error!("{:?}", error);
            std::process::exit(1);
        }
    }

    oid
}
