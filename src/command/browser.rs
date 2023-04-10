use crate::flow::branch::Branch;
use crate::command::runner::Runner;

use super::gitv2::GitV2;

use urlencoding::encode;

pub enum Browser {}

impl Browser {

    pub fn merge_request (branch: &Branch, origin: &Branch) -> Result<String, String> {

        return match GitV2::remote_push() {

            Some(remote_url) => {

                let url = remote_url.replace(".git", "");

                let branch_name = match branch.prefix() {
                    Some(prefix) => format!("{}{}", prefix, branch.name()),
                    None => branch.name().to_string()
                };

                let origin_name = match origin.prefix() {
                    Some(prefix) => format!("{}{}", prefix, origin.name()),
                    None => origin.name().to_string()
                };

                let merge_request_message = format!("Merging branch {} into {}", branch_name, origin_name);
                let merge_request_message = encode(&merge_request_message);

                let url = format!(
                    "{}/merge_requests/new?merge_request[source_branch]={}&merge_request[target_branch]={}&merge_request[title]={}", 
                    url, 
                    branch_name, 
                    origin_name,
                    merge_request_message
                );

                match Runner::open(&url) {
                    Ok(_) => Ok(url),
                    Err(err) => Err(err)
                }

            },
            None => Err(String::from("[ERROR] No remote url found."))

        };

    }

}