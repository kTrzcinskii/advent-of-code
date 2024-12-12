use std::{fs::File, io::Read, iter};

pub fn solve() {
    let input = load_from_file();
    let mut disk = Disk::from(input.as_str());
    let mut second_disk = DiskSecond::from(input.as_str());
    disk.fill_free_spaces();
    println!("First part result: {}", disk.checksum());
    second_disk.fill_free_space();
    println!("Second part result: {}", second_disk.checksum());
}

fn load_from_file() -> String {
    let mut file = File::open("./input/day_09/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DiskFileInfo {
    id: usize,
    length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DiskFreeSpaceInfo {
    length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiskItem {
    File(DiskFileInfo),
    FreeSpace(DiskFreeSpaceInfo),
}

impl DiskItem {
    fn is_free_space(&self) -> bool {
        match self {
            DiskItem::File(_) => false,
            DiskItem::FreeSpace(_) => true,
        }
    }

    fn length(&self) -> usize {
        match self {
            DiskItem::File(disk_file_info) => disk_file_info.length,
            DiskItem::FreeSpace(disk_free_space_info) => disk_free_space_info.length,
        }
    }
}

#[derive(Clone)]
struct Disk {
    items: Vec<DiskItem>,
    max_file_id: usize,
}

impl Disk {
    fn fill_free_spaces(&mut self) {
        let mut last_block_idx = self.items.len() as i32 - 1;
        while self.items[last_block_idx as usize].is_free_space() {
            last_block_idx -= 1;
        }

        let mut first_free_space_idx = 0;
        while !self.items[first_free_space_idx].is_free_space() {
            first_free_space_idx += 1;
        }

        while (first_free_space_idx as i32) < last_block_idx {
            self.items
                .swap(first_free_space_idx, last_block_idx as usize);

            while last_block_idx >= 0 && self.items[last_block_idx as usize].is_free_space() {
                last_block_idx -= 1;
            }

            while first_free_space_idx < self.items.len()
                && !self.items[first_free_space_idx].is_free_space()
            {
                first_free_space_idx += 1;
            }
        }
    }

    fn checksum(&self) -> usize {
        self.items
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, &item)| match item {
                DiskItem::File(disk_file_info) => acc + disk_file_info.id * idx,
                DiskItem::FreeSpace(_) => acc,
            })
    }
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self {
        let mut disk = Disk {
            items: vec![],
            max_file_id: 0,
        };

        for (idx, c) in value.chars().enumerate() {
            let num = c.to_digit(10).unwrap() as usize;
            let item_type = if idx & 1 == 0 {
                if idx / 2 > disk.max_file_id {
                    disk.max_file_id = idx / 2;
                }
                DiskItem::File(DiskFileInfo {
                    id: idx / 2,
                    length: num,
                })
            } else {
                DiskItem::FreeSpace(DiskFreeSpaceInfo { length: num })
            };
            disk.items.extend(iter::repeat(item_type).take(num));
        }

        disk
    }
}

struct DiskSecond {
    items: Vec<DiskItem>,
    max_file_id: usize,
}

impl DiskSecond {
    fn fill_free_space(&mut self) {
        for file_idx in (0..=self.max_file_id).rev() {
            let file_start = self.find_file_start(file_idx);

            let file_len = self.items[file_start].length();

            let mut new_position = 0;
            loop {
                let mut inserted = false;

                if let DiskItem::FreeSpace(fsi) = self.items[new_position] {
                    if fsi.length >= file_len {
                        self.items.swap(file_start, new_position);
                        // Put the block where file was and add merge if its the neighbour of other block
                        self.items[file_start] =
                            DiskItem::FreeSpace(DiskFreeSpaceInfo { length: file_len });
                        if file_start + 1 < self.items.len()
                            && self.items[file_start + 1].is_free_space()
                        {
                            self.items[file_start] = DiskItem::FreeSpace(DiskFreeSpaceInfo {
                                length: file_len + self.items[file_start + 1].length(),
                            });
                            self.items.remove(file_start + 1);
                        }
                        inserted = true;
                    }

                    if fsi.length > file_len {
                        // need to put the part of the free space that still can be used after inserted file
                        self.items.insert(
                            new_position + 1,
                            DiskItem::FreeSpace(DiskFreeSpaceInfo {
                                length: fsi.length - file_len,
                            }),
                        );
                    }
                }

                if inserted || new_position == file_start {
                    break;
                }

                new_position += 1;
            }
        }
    }

    fn find_file_start(&self, file_id: usize) -> usize {
        self.items
            .iter()
            .enumerate()
            .find(|(_, &p)| match p {
                DiskItem::File(disk_file_info) => disk_file_info.id == file_id,
                DiskItem::FreeSpace(_) => false,
            })
            .unwrap()
            .0
    }

    fn checksum(&self) -> usize {
        let mut position = 0;
        let mut checksum = 0;

        for item in &self.items {
            match item {
                DiskItem::File(disk_file_info) => {
                    for _ in 0..disk_file_info.length {
                        checksum += position * disk_file_info.id;
                        position += 1;
                    }
                }
                DiskItem::FreeSpace(disk_free_space_info) => {
                    position += disk_free_space_info.length
                }
            }
        }

        checksum
    }
}

impl From<&str> for DiskSecond {
    fn from(value: &str) -> Self {
        let mut disk = DiskSecond {
            items: vec![],
            max_file_id: 0,
        };

        for (idx, c) in value.chars().enumerate() {
            let num = c.to_digit(10).unwrap() as usize;
            let item_type = if idx & 1 == 0 {
                if idx / 2 > disk.max_file_id {
                    disk.max_file_id = idx / 2;
                }
                DiskItem::File(DiskFileInfo {
                    id: idx / 2,
                    length: num,
                })
            } else {
                DiskItem::FreeSpace(DiskFreeSpaceInfo { length: num })
            };
            disk.items.push(item_type);
        }

        disk
    }
}
