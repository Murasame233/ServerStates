use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use super::{Count, CountsAll, Platform};

pub struct Linux {
    path: PathBuf,
    prev_counts: Option<CountsAll>,
    cores: usize,
    core_status: Vec<i32>,
    all_status: u8,
}

impl Linux {
    fn read_file(&self) -> CountsAll {
        let mut count: CountsAll = CountsAll {
            cores: Vec::new(),
            all: Count { total: 0, used: 0 },
        };
        let file = File::open(&self.path).unwrap();
        let mut reader = BufReader::new(file);
        loop {
            let mut string = String::new();
            reader.read_line(&mut string).unwrap();

            // A single line format: cpu{core} user nice system idle
            let mut s = string.split_ascii_whitespace();
            if let Some(v) = s.next() {
                if v.starts_with("cpu") {
                    let (_, numstr) = v.split_at(3);
                    let used = s.next().unwrap().parse::<u64>().unwrap()
                        + s.next().unwrap().parse::<u64>().unwrap()
                        + s.next().unwrap().parse::<u64>().unwrap();
                    let total = count.all.used + s.next().unwrap().parse::<u64>().unwrap();
                    match numstr.parse::<usize>() {
                        // Have number after 'cpu' means core.
                        Ok(_) => count.cores.push(Count { used, total }),
                        // No number after 'cpu' means all core.
                        Err(_) => {
                            count.all.used = used;
                            count.all.total = total
                        }
                    };
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        count
    }
}

impl Platform for Linux {
    fn init() -> Self {
        let path = Path::new("/proc/stat").to_owned();
        let mut l = Linux {
            path,
            prev_counts: None,
            cores: 0,
            core_status: Vec::new(),
            all_status: 0,
        };
        l.prev_counts = Some(l.read_file());
        l.cores = l.prev_counts.as_ref().unwrap().cores.len();
        l.core_status.resize(l.cores, 0);
        l
    }

    fn get_all(&self) -> &'_ [i32] {
        self.core_status.as_ref()
    }

    fn update(&mut self) {
        let newcount = self.read_file();
        let oldcount = self.prev_counts.as_mut().unwrap();
        self.all_status = ((newcount.all.used - oldcount.all.used) as f32
            / (newcount.all.total - oldcount.all.total) as f32
            * 100f32) as u8;
        for (index, value) in (&mut self.core_status).iter_mut().enumerate() {
            let new = &newcount.cores[index];
            let old = &oldcount.cores[index];
            *value =
                ((new.used - old.used) as f32 / (new.total - old.total) as f32 * 100f32) as i32;
        }
        *oldcount = newcount;
    }

    fn get(&self, core: usize) -> i32 {
        self.core_status[core - 1]
    }
}
