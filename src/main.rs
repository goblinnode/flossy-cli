use clap::{Arg, Command};
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::Command as ProcessCommand;

fn main() {
    let matches = Command::new("flossy")
        .about("cli for adding emojis to commits built with rust")
        .arg(
            Arg::new("commit")
                .short('c')
                .long("commit")
                .num_args(1)
                .help("Create a flossy commit")
        )
        .get_matches();

    if let Some(commit_message) = matches.get_one::<String>("commit") {
      let commit_message = select_emoji_and_create_message(commit_message);
        perform_git_commit(&commit_message);
    } else {
        println!("Use --commit to create a flossy commit.");
    }
}

fn select_emoji_and_create_message(commit_message: &str) -> String {
    let emojis = vec![("✨", "Sparkles"), ("🐛", "Bug (ew!)"), ("📝", "Documentation")];

    let items: Vec<String> = emojis
        .iter()
        .map(|(emoji, description)| format!("{} {}", emoji, description))
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an emoji")
        .default(0)
        .items(&items)
        .interact()
        .unwrap();

    let (emoji, _) = emojis[selection];
    let commit_message = format!("{} {}", emoji, commit_message);

    println!("Commit message: {}", commit_message);
    commit_message
}

fn perform_git_commit(commit_message: &str) {
  let status = ProcessCommand::new("git")
      .arg("commit")
      .arg("-m")
      .arg(commit_message)
      .status()
      .expect("Failed to execute git command");

    if status.success() {
        println!("Commit successful!");
    } else {
        println!("Commit failed. Please check your git status.");
    }
}