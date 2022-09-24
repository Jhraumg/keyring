use keyring::Entry;
use clap::Parser;

use eyre::{Result, Context};


#[derive(Parser, Debug)]
struct GetEntryProps{
    /// secret name
    #[clap()]
    secret_name : String,
}

#[derive(Parser, Debug)]
struct SetEntryProps{
    /// secret name
    #[clap(value_parser)]
    secret_name : String,
    
    /// secret . If none is provided, will be asked for interactively
    #[clap()]
    secret : Option<String>,
}

#[derive(Parser, Debug)]
enum Command {
    /// set / update a secret
    #[clap()]
    Set(SetEntryProps),

    /// retrieve a secret
    #[clap()]
    Get(GetEntryProps)
}

/// Load/Store secrets from/into keyring
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
   /// command
   #[clap(subcommand)]
   command: Command,
}


fn new_entry(secret_name : &str) -> Entry {
    Entry::new_with_target(secret_name, "service", "username")
}

fn get_secret(secret_name : &str)-> Result<()>{
    let entry = new_entry(secret_name);
    let secret = entry.get_password().wrap_err_with(|| format!("could not read {secret_name}"))?;
    print!("{secret}");
    Ok(())
}

fn set_secret(secret_name : &str, secret : Option<String>)->Result<()>{

    let secret = match secret {
        Some(secret) => secret,
        None => rpassword::prompt_password("Type secret value: ")?,
    };

    write_secret(secret_name, &secret)
}

fn write_secret(secret_name : &str, secret : &str)-> Result<()>{
    let entry = new_entry(secret_name);
    entry.set_password(secret).wrap_err_with(|| format!("could not write {secret_name}"))?;
    Ok(())
}


fn main() -> Result<()> {
    let args: Args = Args::parse();
    match args.command {
        Command::Set(setopts) => set_secret(&setopts.secret_name, setopts.secret)?,
        Command::Get(getopts) => get_secret(&getopts.secret_name)?,
    }

    Ok(())
}