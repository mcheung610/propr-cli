use std::process;

use clap::ArgMatches;

use crate::utils::{github, loader, openai};

pub async fn run(sub_matches: &ArgMatches) {
    let current_branch = github::get_current_branch();
    let default_branch = github::get_default_branch();

    let diff = github::get_diff(&default_branch);
    if diff.is_empty() {
        println!("No diff found");
        return;
    }

    let mut loader = loader::create_loader("Generating title");
    match openai::generate_title_with_branch_name(&diff, &current_branch).await {
        Ok(title) => {
            loader.stop_with_message("✅ Done\n".into());
            println!("{}", title);
        }
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    }
}