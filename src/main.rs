mod item_name;
mod ug_item_table;

use crate::item_name::ItemNameTable;
use crate::ug_item_table::UgItemTable;
use bdsp_dig_generator::xorshift::XorShift;
use bdsp_dig_generator::{run_results, Version};
use clap::{Parser, Subcommand, ValueEnum};

const TYPES_EN: &str = include_str!("../types_en.txt");
const UG_ITEM_TABLE_RAW: &str = include_str!("../UgItemTable.json");
const ITEM_NAME_RAW: &str = include_str!("../english_ss_itemname.json");
const UG_ITEM_NAME_RAW: &str = include_str!("../english_dp_underground_name.json");

fn load_string_list(list: &str) -> Vec<&str> {
    list.split('\n')
        .map(|s| {
            if s.is_empty() {
                s
            } else if s.as_bytes()[s.len() - 1] == b'\r' {
                &s[..(s.len() - 1)]
            } else {
                s
            }
        })
        .collect()
}

#[derive(Parser)]
struct Cli {
    #[clap(value_enum)]
    version: GameVersion,
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum GameVersion {
    BD,
    SP,
}

impl From<GameVersion> for Version {
    fn from(gv: GameVersion) -> Self {
        match gv {
            GameVersion::BD => Version::BD,
            GameVersion::SP => Version::SP,
        }
    }
}

fn main() {
    let types = load_string_list(TYPES_EN);
    let ug_item_table = serde_json::from_str::<UgItemTable>(UG_ITEM_TABLE_RAW).unwrap();
    let item_name = serde_json::from_str::<ItemNameTable>(ITEM_NAME_RAW).unwrap();
    let item_name_ug = serde_json::from_str::<ItemNameTable>(UG_ITEM_NAME_RAW).unwrap();

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
                cli.version.into(),
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
                if item.0 < 1000 {
                    let item_id = ug_item_table
                        .table
                        .iter()
                        .find(|i| i.ug_item_id == item.0 as i32)
                        .unwrap();
                    let item_name = if item_id.item_table_id == -1 {
                        &item_name_ug
                            .label_data_array
                            .iter()
                            .find(|f| f.label_index == item_id.ug_item_id)
                            .unwrap()
                            .word_data_array
                            .first()
                            .unwrap()
                            .str
                    } else {
                        &item_name
                            .label_data_array
                            .iter()
                            .find(|f| f.label_index == item_id.item_table_id)
                            .unwrap()
                            .word_data_array
                            .first()
                            .unwrap()
                            .str
                    };

                    println!(
                        "{} - Item ID: {} @ ({},{})",
                        char::from(character),
                        item_name,
                        item.1,
                        item.2
                    );
                } else {
                    let id = (item.0 % 1000) / 10;
                    let box_type = match item.0 % 10 {
                        1 => "Pretty",
                        _ => "Gorgeous",
                    };
                    println!(
                        "{} - {} Stone Box: {} @ ({},{})",
                        char::from(character),
                        box_type,
                        types[id as usize],
                        item.1,
                        item.2
                    );
                }

                character += 1;
            }
        }
        Subcommands::Results { item_id } => {
            let results = run_results(
                cli.version.into(),
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
                    let item_id = ug_item_table
                        .table
                        .iter()
                        .find(|i| i.ug_item_id == item.0 as i32)
                        .unwrap();
                    let item_name = if item_id.item_table_id == -1 {
                        &item_name_ug
                            .label_data_array
                            .iter()
                            .find(|f| f.label_index == item_id.ug_item_id)
                            .unwrap()
                            .word_data_array
                            .first()
                            .unwrap()
                            .str
                    } else {
                        &item_name
                            .label_data_array
                            .iter()
                            .find(|f| f.label_index == item_id.item_table_id)
                            .unwrap()
                            .word_data_array
                            .first()
                            .unwrap()
                            .str
                    };

                    println!("Item: {} @ ({},{})", item_name, item.1, item.2);
                }
                println!()
            }
        }
    }
}
