use hwt905_485_rs::Hwt905;
use std::thread::sleep;
use std::time::Duration;

struct Args {
    port: String,
    baud: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();
    let args = Args {
        port: args
            .opt_value_from_str("--port")?
            .unwrap_or("/dev/ttyUSB0".to_string()),
        baud: args.opt_value_from_str("--baud")?.unwrap_or(9600),
    };

    let mut hwt905 = Hwt905::new(&args.port, 0x50, args.baud)?;
    loop {
        // sleep(Duration::from_millis(500));
        let mf = hwt905.magnetic_field()?;
        println!("magnetic field: x {}, y {}, z {}", mf.0, mf.1, mf.2);
    }
}
