use std::process::exit;

use git2::{Oid, Repository, Revwalk};

use crate::model::Commit;

pub fn get_commit_messages_till_head_from(
    from_commit_hash: Option<git2::Oid>,
    from_tag: Option<String>,
) -> Vec<Commit> {
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
                exit(crate::ERROR_EXIT_CODE);
            }
        }
    }

    error!("Provide either the --from-tag or --from-commit-hash argument.");
    exit(crate::ERROR_EXIT_CODE);
}

fn get_commit_messages_till_head_from_oid(from_commit_hash: Oid) -> Vec<Commit> {
    let mut commits = vec![];

    let repository = get_repository();
    let mut revwalk = get_revwalk(&repository, from_commit_hash);

    loop {
        match revwalk.next() {
            Some(Ok(oid)) => match get_commit_message(&repository, oid) {
                Some(message) => {
                    trace!("Found commit '{}'s message '{:?}'.", oid, message);
                    commits.push(Commit { oid, message });
                }
                None => {
                    warn!("Commit hash '{}' has no message.", oid);
                }
            },
            Some(Err(error)) => {
                error!("Revwalk threw error while walking the commits.");
                error!("{:?}", error);
                exit(crate::ERROR_EXIT_CODE);
            }
            None => break,
        }
    }

    debug!("'{}' commit messages in the vector.", commits.len());
    commits.reverse();
    commits
}

fn get_revwalk(repository: &Repository, from_commit_hash: Oid) -> Revwalk {
    match repository.revwalk() {
        Ok(mut revwalk) => {
            match revwalk.push_head() {
                Ok(_result) => {}
                Err(_error) => {
                    error!("Unable to push HEAD onto the revwalk.");
                    exit(crate::ERROR_EXIT_CODE);
                }
            }

            match revwalk.hide(from_commit_hash) {
                Ok(_result) => {}
                Err(_error) => {
                    error!(
                        "Can not find commit hash '{}' in the Git repository.",
                        from_commit_hash
                    );
                    exit(crate::ERROR_EXIT_CODE);
                }
            }

            revwalk
        }
        Err(error) => {
            error!("Unable to get revwalk from local Git repository.");
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

fn get_commit_message(repository: &Repository, oid: Oid) -> Option<String> {
    match repository.find_commit(oid) {
        Ok(commit) => commit.message().map(|m| m.to_string()),

        Err(_error) => {
            error!("Can not find commit '{}' in current repository.", oid);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

fn get_repository() -> Repository {
    match Repository::open_from_env() {
        Ok(repository) => repository,
        Err(error) => {
            error!("Unable to open the Git repository.");
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }
}

fn get_tag_oid(matching: &str) -> Option<Oid> {
    let mut oid: Option<Oid> = None;
    let repository = get_repository();
    let matching = format!("refs/tags/{}", matching);

    match repository.tag_foreach(|tag_oid: Oid, tag_name: &[u8]| -> bool {
        match std::str::from_utf8(tag_name) {
            Ok(tag_name) => {
                if tag_name == matching {
                    debug!("Matching '{}' at commit id '{}'.", tag_name, tag_oid);
                    oid = Some(tag_oid);
                    return false;
                }
            }
            Err(error) => {
                error!("Unable to parse String from tag's name.");
                error!("{:?}", error);
                exit(crate::ERROR_EXIT_CODE);
            }
        }

        true
    }) {
        Ok(_) => {}
        Err(error) => {
            error!("Unable to perform function on all tags.");
            error!("{:?}", error);
            exit(crate::ERROR_EXIT_CODE);
        }
    }

    oid
}
