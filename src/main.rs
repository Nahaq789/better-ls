use std::env;

pub mod structs;

const KEY_SET: &'static [&'static str] = &[
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u",
    "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "L", "M", "N", "O", "P",
    "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
];

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let paths = std::fs::read_dir(path)?;

    // first line cd ../
    println!("key: {}  ../", KEY_SET[0]);
    for (i, p) in paths.into_iter().enumerate() {
        match p {
            Ok(it) => {
                let folder = &it.file_name();
                let file_type = it.file_type()?;

                if file_type.is_file() {
                    continue;
                }
                println!("key: {}  {}", KEY_SET[i + 1], folder.to_str().unwrap());
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }

        if i == 6 {
            println!("load more...");
            let mut command = String::new();
            std::io::stdin().read_line(&mut command).ok();
        }
    }

    let mut command = String::new();
    std::io::stdin().read_line(&mut command).ok();

    println!("{}", command);
    Ok(())
}
