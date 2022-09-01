use crate::deposit_item_data::DepositItemData;
use crate::stone_box_data::StoneBoxData;
use crate::xorshift::XorShift;

mod deposit_item_data;
mod stone_box_data;
pub mod xorshift;

const STONE_BOX_DATA_RAW: &str = include_str!("../StoneBoxRawData.json");
const DEPOSIT_ITEM_DATA_RAW: &str = include_str!("../DepositItemRawData.json");

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Version {
    BD = 0,
    SP = 1,
}

#[derive(Debug)]
struct DepositItem {
    item_id: u32,
    ratio: i32,
    width: usize,
    height: usize,
    shape: Vec<u8>,
}

fn rotate_right(original: &Vec<u8>, height: usize, width: usize) -> (Vec<u8>, usize, usize) {
    let mut new = vec![0; original.len()];
    for i in 0..height {
        let shift = height.wrapping_add(!i);
        for j in 0..width {
            let original_index = j + i * width;
            let new_index = shift + j * height;
            let old = original[original_index];
            new[new_index] = old;
        }
    }
    (new, width, height)
}

fn rotate_right_multiple(
    original: &[u8],
    mut height: usize,
    mut width: usize,
    count: i32,
) -> Vec<u8> {
    let mut new = original.to_owned();
    for _ in 0..count {
        let (rotated, new_height, new_width) = rotate_right(&new, height, width);
        new = rotated;
        height = new_height;
        width = new_width;
    }
    new
}

#[allow(clippy::type_complexity)]
pub fn run_results(
    version: Version,
    mut rng: XorShift,
    diglett: bool,
    sp_cleared: bool,
    national_dex: bool,
    initial_advances: usize,
    max_advances: usize,
) -> Vec<(
    Vec<(u32, u32, u32, usize, usize, Vec<u8>)>,
    [[char; 13]; 10],
)> {
    let mut results = Vec::with_capacity(max_advances);

    if initial_advances > 0 {
        rng.advance(initial_advances);
    }

    let stone_box_data = serde_json::from_str::<StoneBoxData>(STONE_BOX_DATA_RAW).unwrap();
    let deposit_item_data = serde_json::from_str::<DepositItemData>(DEPOSIT_ITEM_DATA_RAW).unwrap();

    let mut deposit_items = Vec::with_capacity(deposit_item_data.deposit.len() * 4);

    deposit_item_data.deposit.iter().for_each(|d| {
        let height = d.shape.split('.').count() - 1;
        let width = d.shape.split('.').next().unwrap().len();
        let shape: Vec<u8> = d
            .shape
            .as_bytes()
            .iter()
            .filter_map(|&b| if b == b'.' { None } else { Some(b) })
            .collect();

        let rate = match version {
            Version::BD => {
                if national_dex {
                    d.ratio3
                } else if sp_cleared {
                    d.ratio2
                } else {
                    d.ratio1
                }
            }
            Version::SP => {
                if national_dex {
                    d.ratio6
                } else if sp_cleared {
                    d.ratio5
                } else {
                    d.ratio4
                }
            }
        };

        let ratio = if d.turn > 0 {
            rate / (d.turn + 1)
        } else {
            rate
        };
        let item_id = d.item_id;
        deposit_items.push(DepositItem {
            item_id,
            ratio,
            width,
            height,
            shape: shape.clone(),
        });

        if d.turn > 0 {
            for i in 1..=d.turn {
                deposit_items.push(DepositItem {
                    item_id,
                    ratio,
                    width: if i % 2 == 1 { height } else { width },
                    height: if i % 2 == 1 { width } else { height },
                    shape: rotate_right_multiple(&shape, height, width, i),
                });
            }
        }
    });

    for _ in 0..=max_advances {
        let mut character = b'A';
        let mut map = [['#'; 13]; 10];
        let mut clone = rng;

        clone.advance(50);

        let mut item_count = clone.rand_range(2, 5);

        let mut result = Vec::with_capacity(item_count as usize);

        let mut box_type = 0;
        let mut box_id = 0;
        let stone_box_rand_max = stone_box_data
            .r#box
            .iter()
            .map(|b| if diglett { b.ratio2 } else { b.ratio1 })
            .sum::<i32>();
        if diglett || clone.rand_range(0, 100) < 50 {
            let mut statue_box_rand = clone.rand_range(0, stone_box_rand_max as u32) as i32;
            for b in &stone_box_data.r#box {
                statue_box_rand -= if diglett { b.ratio2 } else { b.ratio1 };
                if statue_box_rand < 0 {
                    box_type = b.r#type;
                    box_id = b.box_id;
                    break;
                }
            }
            item_count -= 1;
            let mut x = clone.rand_range(0, 0xd);
            let mut y = clone.rand_range(0, 10);
            while x + 3 > 0xd || y + 3 > 10 {
                x = clone.rand_range(0, 0xd);
                y = clone.rand_range(0, 10);
            }
            let shape = vec![b'x'; 9];
            place_item(&mut map, x, y, 3, 3, &shape, character);
            character += 1;
            result.push((1000 + box_type * 10 + box_id, x, y, 3usize, 3usize, shape));
        }

        let rand_max = deposit_items.iter().map(|i| i.ratio).sum::<i32>() as u32;

        while item_count > 0 {
            'inner: loop {
                let mut temp = clone.rand_range(0, rand_max) as i32;
                let mut item_id = 0;
                let mut width = 0;
                let mut height = 0;
                let mut shape = Vec::new();
                for dep in deposit_items.iter() {
                    temp -= dep.ratio;
                    if temp < 0 {
                        item_id = dep.item_id;
                        width = dep.width;
                        height = dep.height;
                        shape = dep.shape.clone();
                        break;
                    }
                }
                let x = clone.rand_range(0, 0xd);
                let y = clone.rand_range(0, 10);
                if x + width as u32 <= 0xd
                    && y + height as u32 <= 10
                    && !overlaps(&map, x, y, width, height, &shape)
                {
                    item_count -= 1;
                    place_item(&mut map, x, y, width, height, &shape, character);
                    character += 1;
                    result.push((item_id, x, y, height, width, shape));
                    break 'inner;
                }
            }
        }
        results.push((result, map));
        rng.next();
    }

    results
}

fn overlaps(
    map: &[[char; 13]; 10],
    x: u32,
    y: u32,
    width: usize,
    height: usize,
    shape: &[u8],
) -> bool {
    for i in y..(y + height as u32) {
        for j in x..(x + width as u32) {
            if shape[((j - x) as usize) + ((i - y) as usize) * width] == b'x'
                && map[i as usize][j as usize] != '#'
            {
                return true;
            }
        }
    }
    false
}

fn place_item(
    map: &mut [[char; 13]; 10],
    x: u32,
    y: u32,
    width: usize,
    height: usize,
    shape: &[u8],
    character: u8,
) {
    for i in y..(y + height as u32) {
        for j in x..(x + width as u32) {
            if shape[((j - x) as usize) + ((i - y) as usize) * width] == b'x' {
                map[i as usize][j as usize] = char::from(character);
            }
        }
    }
}
