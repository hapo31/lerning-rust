extern crate rand;

use rand::Rng;
use std::thread;
use std::time::Duration;

struct Field {
    width: usize,
    height: usize,
    data: Vec<i32>,
}

impl Field {
    fn new(width: usize, height: usize) -> Field {
        Field {
            height: height,
            width: width,
            data: vec![],
        }
    }

    // self のデータに対して変更を加える場合、&mut self とする
    fn init_field(&mut self) {
        self.data.clear();
        for y in 0..self.height {
            for x in 0..self.width {
                if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                    self.data.push(0);
                } else {
                    self.data.push(rand::thread_rng().gen_range(0, 2));
                }
            }
        }
    }

    fn update(&mut self) {
        let old_data = self.data.clone();

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let mut count = 0;
                let pos = y * self.width + x;

                // 上
                if old_data[pos - self.width] == 1 {
                    count += 1;
                }

                // 下
                if old_data[pos + self.width] == 1 {
                    count += 1;
                }

                // 右
                if old_data[pos + 1] == 1 {
                    count += 1;
                }

                // 左
                if old_data[pos - 1] == 1 {
                    count += 1;
                }

                // 左上
                if old_data[pos - self.width - 1] == 1 {
                    count += 1;
                }

                // 右上
                if old_data[pos - self.width + 1] == 1 {
                    count += 1;
                }

                // 左下
                if old_data[pos + self.width - 1] == 1 {
                    count += 1;
                }

                // 右下
                if old_data[pos + self.width + 1] == 1 {
                    count += 1;
                }

                if old_data[pos] == 1 {
                    if count == 2 || count == 3 {
                        self.data[pos] = 1;
                    } else {
                        self.data[pos] = 0;
                    }
                } else {
                    if count == 3 {
                        self.data[pos] = 1;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut field = Field::new(25, 25);
    field.init_field();

    let mut count = 0;

    loop {
        field.update();

        for cell in &field.data {
            if count != 0 && count % field.width == 0 {
                println!("");
            }
            print!(
                "{} ",
                match cell {
                    1 => "■",
                    _ => "□",
                }
            );
            count = count + 1;
        }

        println!("\n----------------------------------");

        thread::sleep(Duration::from_millis(500));
    }
}
