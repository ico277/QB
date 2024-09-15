use clap::{value_parser, Parser};
use std::process::{Command, Stdio};
use std::thread;
use std::thread::JoinHandle;
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[command(version = "0.2.0-dev", about = None, long_about = None)]
struct Args {
    /// How many iterations to run
    #[arg(short, long = "iters", default_value_t = 20, value_parser = value_parser!(u32).range(1..))]
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

    /// Enable threading and the amount of threads to use
    #[arg(short, long, value_parser = value_parser!(u32).range(2..))]
    threads: Option<u32>,
}

fn run_cmd<S, I>(args: I) -> u128
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut args = args.into_iter();

    // create command
    let mut binding = Command::new(args.next().unwrap());
    let cmd = binding.args(args);
    let cmd = cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    // create timestamp
    let now = SystemTime::now();

    // run command
    let _status = cmd.status().expect("failed to run command!");

    // return time elapsed in milliseconds
    now.elapsed()
        .expect("failed to get elapsed time!")
        .as_millis()
}

fn run_cmd_threads<S, I>(cmd: I) -> JoinHandle<u128>
where
    I: IntoIterator<Item = S> + Send + 'static,
    S: AsRef<std::ffi::OsStr> + Send + 'static,
{
    thread::spawn(move || run_cmd(cmd))
}

fn main() {
    // parse commnad line arguments into Args using clap
    let args = Args::parse();
    // parse iterations
    let mut iterations = args.iterations;
    // parse cmd args
    let cmd = args.cmd.clone();

    // run iterations
    let mut total_time: u128 = 0;
    // threading
    if let Some(threads) = args.threads {
        let mut ran_iterations = 0;
        while ran_iterations < iterations {
            let mut handles: Vec<JoinHandle<u128>> = Vec::with_capacity(threads as usize);
            for i in 0..threads {
                println!("Running iteration on thread {}", i + 1);
                let cmd_clone = cmd.clone(); // Clone `cmd` for each thread
                                             // run command
                let cmd_to_run = if args.nullpipe {
                    vec![
                        "sh".to_string(),
                        "-c".to_string(),
                        cmd_clone + " 2>&1 > /dev/null",
                    ]
                } else if args.shell {
                    vec!["sh".to_string(), "-c".to_string(), cmd_clone]
                } else {
                    vec![cmd_clone]
                };

                handles.push(run_cmd_threads(cmd_to_run))
            }
            for handle in handles {
                total_time += handle.join().unwrap();
                ran_iterations += 1;
            }
        }
        iterations = ran_iterations;
    }
    // single threading
    else {
        for i in 0..iterations {
            println!("Running iteration {}", i + 1);
            // run command
            if args.nullpipe {
                total_time += run_cmd(vec!["sh", "-c", format!("{cmd} 2>&1 > /dev/null").as_str()]);
            } else if args.shell {
                total_time += run_cmd(vec!["sh", "-c", cmd.as_str()]);
            } else {
                total_time += run_cmd(vec![cmd.clone()]);
            }
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
