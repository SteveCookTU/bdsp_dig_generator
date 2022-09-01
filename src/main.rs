use bdsp_dig_generator::xorshift::XorShift;
use bdsp_dig_generator::{run_results, Version};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    s0: String,
    s1: String,
    s2: String,
    s3: String,
    advances: usize,
    #[clap(short, long)]
    diglett: bool,
    #[clap(short, long)]
    sp_cleared: bool,
    #[clap(short, long)]
    national_dex: bool,
    #[clap(subcommand)]
    option: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    Map,
    Results {
        #[clap(
            help = "Item ID to filter for. Box IDs are 1000 + (type_id * 10) + rarity. rarity is 1 for pretty and 2 for gorgeous"
        )]
        item_id: Option<u32>,
    },
}

fn main() {
    let cli: Cli = Cli::parse();
    println!("Advances: {}", cli.advances);
    let s0 = u32::from_str_radix(&cli.s0, 16).unwrap();
    let s1 = u32::from_str_radix(&cli.s1, 16).unwrap();
    let s2 = u32::from_str_radix(&cli.s2, 16).unwrap();
    let s3 = u32::from_str_radix(&cli.s3, 16).unwrap();
    println!("s0: {:#08X}", s0);
    println!("s1: {:#08X}", s1);
    println!("s2: {:#08X}", s2);
    println!("s3: {:#08X}", s3);
    println!();

    let rng = XorShift::from_state([s0, s1, s2, s3]);

    println!();
    match cli.option {
        Subcommands::Map => {
            let result = run_results(
                Version::BD,
                rng,
                cli.diglett,
                cli.sp_cleared,
                cli.national_dex,
                cli.advances,
                1,
            );
            for ch in result.first().unwrap().1 {
                println!("{}", ch.iter().collect::<String>());
            }
            println!();
            let mut character = b'A';
            for item in &result.first().unwrap().0 {
                println!(
                    "{} - Item ID: {} @ ({},{})",
                    char::from(character),
                    item.0,
                    item.1,
                    item.2
                );
                character += 1;
            }
        }
        Subcommands::Results { item_id } => {
            let results = run_results(
                Version::BD,
                rng,
                cli.diglett,
                cli.sp_cleared,
                cli.national_dex,
                0,
                cli.advances,
            );
            for (i, (result, _)) in results.iter().enumerate() {
                if let Some(id) = item_id {
                    if !result.iter().any(|r| r.0 == id) {
                        continue;
                    }
                }
                println!("Advances: {i}");
                for item in result {
                    println!("Item ID: {} @ ({},{})", item.0, item.1, item.2);
                }
                println!()
            }
        }
    }
}
