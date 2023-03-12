use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("pngme")
        .about("Put a secret message into a PNG file")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("encode")
                .about("Encode secret message in PNG file")
                .arg(arg!(<PATH> "The PNG file to encode"))
                .arg(arg!(<CHUNK_TYPE> "The 4 byte chunk type code"))
                .arg(arg!(<MESSAGE> "The secret message to encode"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("decode")
                .about("Decode secret message in PNG file")
                .arg(arg!(<PATH> "The PNG file to encode"))
                .arg(arg!(<CHUNK_TYPE> "The 4 byte chunk type code"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove secret message in PNG file")
                .arg(arg!(<PATH> "The PNG file to encode"))
                .arg(arg!(<CHUNK_TYPE> "The 4 byte chunk type code"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("print")
                .about("Print secret message in PNG file")
                .arg(arg!(<PATH> "The PNG file to encode"))
                .arg_required_else_help(true),
        )
}
