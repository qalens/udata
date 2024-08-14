use std::io::Read;
use clap::{Parser, Subcommand};
use jsonpath_rust::JsonPath;
use serde_json::Value;

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct UDataCLI {
    #[command(subcommand)]
    command: Commands
}
#[derive(Subcommand)]
enum Commands{
    J {
        #[arg(short,long)]
        path: Option<String>,
    }
}
fn main() {
    let udata_cli = UDataCLI::parse();
    match &udata_cli.command {
        Commands::J { path}=>{
            let pth=if let Some(p)=path {
                Option::Some(JsonPath::try_from(p.as_str()).expect("provided path is not valid json path"))
            } else {
                Option::None
            };
            let mut input =  Vec::new();
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_end(&mut input);
            let jstr=String::from_utf8(input).expect("Our bytes should be valid utf8");
            let json=serde_json::from_str::<Value>(jstr.as_str()).expect("provided data is not valid json");
            if let Some(path)=pth{
                let v = path.find(&json);
                println!("{}",serde_json::to_string_pretty(&v).expect("Output is not json"))
            }

        }
    }
}
