use keyring::Entry;
use clap::Parser;

use eyre::{Result, Context};


#[derive(Parser, Debug)]
struct GetEntryProps{
    
    /// service name
    #[arg()]
    service_name : String,
    
    /// user name
    #[arg()]
    user_name : String,
}

#[derive(Parser, Debug)]
struct SetEntryProps{
    /// service name
    #[arg()]
    service_name : String,
    
    /// user name
    #[arg()]
    user_name : String,
    
    /// secret . If none is provided, will be asked for interactively
    #[arg()]
    secret : Option<String>,
}

#[derive(Parser, Debug)]
enum Command {
    /// set / update a secret
    #[arg()]
    Set(SetEntryProps),

    /// retrieve a secret
    #[arg()]
    Get(GetEntryProps)
}

/// Load/Store secrets from/into keyring
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
   /// command
   #[command(subcommand)]
   command: Command,
}


fn new_entry(service_name : &str, user_name : &str) -> Result<Entry> {
    Entry::new(service_name, user_name).wrap_err_with(||format!("could not select entry '{service_name}' for user '{user_name}'/"))
}

fn get_secret(service_name : &str, user_name : &str)-> Result<()>{
    let entry = new_entry(service_name, user_name)?;
    let secret = entry.get_password().wrap_err_with(|| format!("could not read '{service_name}' '{user_name}'"))?;
    println!("{secret}");
    Ok(())
}

fn set_secret(service_name : &str, user_name : &str, secret : Option<String>)->Result<()>{

    let secret = match secret {
        Some(secret) => secret,
        None => rpassword::prompt_password("Type secret value: ")?,
    };

    write_secret(service_name, user_name, &secret)
}

fn write_secret(service_name : &str, user_name : &str, secret : &str)-> Result<()>{
    let entry = new_entry(service_name, user_name)?;
    entry.set_password(secret).wrap_err_with(|| format!("could not write {service_name} {user_name}"))?;
    Ok(())
}


fn main() -> Result<()> {
    let args: Args = Args::parse();
    match args.command {
        Command::Set(setopts) => set_secret(&setopts.service_name, &setopts.user_name, setopts.secret)?,
        Command::Get(getopts) => get_secret(&getopts.service_name, &getopts.user_name)?,
    }

    Ok(())
}