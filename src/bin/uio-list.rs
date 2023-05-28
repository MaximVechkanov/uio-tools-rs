use clap::Parser;

/// Print uio devices in the systems
#[derive(Parser)]
struct CliParams {

    /// Pattern to search for in the device name
    #[arg(short, long)]
    name: Option<String>,

    /// UIO device index for which to show properties
    #[arg(short, long)]
    index: Option<u32>,

}

const UIO_CLASS_PATH: &str = "/sys/class/uio";

fn print_properties(uio_name: String)
{
    let path = UIO_CLASS_PATH.to_owned() + "/" + &uio_name;
    println!("{}:", uio_name);

    let name_filename = path.clone() + "/name";

    let name_res = std::fs::read_to_string(&name_filename);

    let name = match name_res {
        Err(err) => {
            println!("Failed to read uio name from '{}', err = {}", name_filename, err);
            return;
        },
        Ok(name) => name,
    };

    print!("  name: {}", name);

    let maps_res = std::fs::read_dir(path + "/maps");

    let maps_dir = match maps_res {
        Err(..) => {
            println!("  Have no maps");
            return
        },
        Ok(res) => res,
    };

    println!("  maps:");

    for map_dir in maps_dir {
        let addr = std::fs::read_to_string(map_dir.as_ref().unwrap().path().to_str().unwrap().to_owned() + "/addr").unwrap();
        print!("    {}: {}", map_dir.unwrap().file_name().to_str().unwrap(), addr);

    }
}

fn main() {
    

    let dir_res = std::fs::read_dir(UIO_CLASS_PATH);

    let dir = match dir_res {
        Err(error) => { println!("Cannot read directory '{}', err = {}", UIO_CLASS_PATH, error); return; },
        Ok(dir) => dir,
    };

    if dir.count() == 0 {
        println!("UIO class folder is empty, no UIO devices found");
        return;
    }

    let args = CliParams::parse();

    if args.index.is_some() {
        print_properties("/uio".to_owned() + &args.index.unwrap().to_string());
        return;
    }

    // Recreate iterator, unwrap whithout comments
    let dir = std::fs::read_dir(UIO_CLASS_PATH).unwrap();

    for uio_dir in dir {
        // uio0, uio1, ...
        print_properties(uio_dir.unwrap().file_name().into_string().unwrap());
        for _ in 0..40 {
            print!("-");
        }
        println!();
    }
}