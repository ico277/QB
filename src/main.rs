use clap::Parser;
use std::process::{Command, Stdio};
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[command(version = "0.1.0-dev", about = None, long_about = None)]
struct Args {
    /// How many iterations to run
    #[arg(short, long = "iters", default_value_t = 20)]
    iterations: u32,

    /// Run cmd with shell
    #[arg(short, long, default_value_t = false)]
    shell: bool,

    /// Piping output to /dev/null (infers --shell)
    #[arg(short, long, default_value_t = false)]
    nullpipe: bool,

    /// The command to bench
    #[arg(short, long)]
    cmd: String,
}

fn run_cmd(cmd: &mut Command) -> u128 {
    // create timestamp
    let now = SystemTime::now();

    // run command
    let _status = cmd.status().expect("failed to run command!");

    // return time elapsed in milliseconds
    now.elapsed()
        .expect("failed to get elapsed time!")
        .as_millis()
}

fn main() {
    // parse commnad line arguments into Args using clap
    let args = Args::parse();
    // parse iterations
    let iterations = args.iterations;
    // parse cmd args
    let cmd = args.cmd;

    // run iterations
    let mut total_time: u128 = 0;
    for i in 0..iterations {
        println!("Running iteration {}", i + 1);
        if args.nullpipe || args.shell {
            // create command
            let mut binding = Command::new("sh");
            let command: &mut Command;
            if args.nullpipe {
                command = binding.arg("-c").arg(cmd.clone() + " 2>&1 > /dev/null");
            } else {
                command = binding.arg("-c").arg(cmd.clone());
            }
            // pipe output to stdout
            let cmd = command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
            // run command
            total_time += run_cmd(cmd);
        } else {
            // create command
            let mut binding = Command::new(cmd.clone());
            // pipe output to stdout
            let cmd = binding.stdout(Stdio::inherit()).stderr(Stdio::inherit());
            // run command
            total_time += run_cmd(cmd);
        }
    }
    // calculate (convering to milliseconds aswell) and print average
    println!(
        "Iterations: {}\nTotal time: ~{:.3}s\nAverage time: ~{:.3}s",
        iterations,
        total_time as f64 / 1000f64,
        (total_time as f64 / iterations as f64) / 1000f64
    );
}
