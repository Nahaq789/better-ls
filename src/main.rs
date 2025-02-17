use clap::Parser;
use structs::Cli;

pub mod structs;

fn main() -> anyhow::Result<()> {
    // let args = Cli::parse();
    // println!("{:?}, {:?}", args.path(), args.pattern());
    // let current = std::fs::read_to_string(args.path()).expect("could not read file");
    // println!("{:?}", current);
    //
    // for line in current.lines() {
    //     if line.contains(args.pattern()) {
    //         println!("{:?}", line)
    //     }
    // } 

    // let mut command = std::process::Command::new("ls");
    // command
    //     .spawn()
    //     .expect("ls command failed to start");
    //
    // let output = match command.output() {
    //     Ok(o) => o,
    //     Err(_) => panic!("error")
    // };
    // println!("{:?}", output)
    //
    let paths = std::fs::read_dir("./")?;
    for path in paths {
        match path {
            Ok(it) => {
                let folder = &it.file_name();
                let file_type = it.file_type()?;
                
                if file_type.is_file() {
                    continue;
                }
                println!("{}, {:?}", folder.to_str().unwrap(), file_type);
            },
            Err(e) => {
                println!("{:?}", e)
            }
        } 
    }
    Ok(())
}
