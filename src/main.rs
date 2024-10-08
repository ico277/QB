use clap::{value_parser, Parser};
use std::process::{Command, Stdio};
use std::thread;
use std::thread::JoinHandle;
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"), about = None, long_about = None)]
struct Args {
    /// How many iterations to run
    #[arg(short, long = "iters", default_value_t = 20, value_parser = value_parser!(u32).range(1..))]
    iterations: u32,

    /// Run cmd with shell
    #[arg(short, long, default_value_t = false)]
    shell: bool,

    /// Piping output to /dev/null (infers --shell)
    /// (!!! Does not do anything under windows for now !!!)
    #[arg(short, long, default_value_t = false)]
    nullpipe: bool,

    /// The command to bench
    #[arg(short, long)]
    cmd: String,

    /// Enable threading and the amount of threads to use
    #[arg(short, long, value_parser = value_parser!(u32).range(2..))]
    threads: Option<u32>,
}

fn run_cmd(cmd: String, nullpipe: bool, shell: bool) -> u128 {
    // generate the command args with the given cli arguments
    let cmd_to_run = if nullpipe || shell {
        #[cfg(unix)]
        {
            "sh".to_string()
        }
        #[cfg(target_os = "windows")]
        {
            "cmd.exe".to_string()
        }
    } else {
        cmd.clone()
    };
    let args = if nullpipe {
        #[cfg(unix)]
        {
            Some(vec!("-c".to_string(), format!("{cmd} 2>&1 > /dev/null")))
        }
        #[cfg(target_os = "windows")]
        {
            Some(vec!("/c".to_string(), cmd))
        }
    } else if shell {
        #[cfg(unix)]
        {
            Some(vec!("-c".to_string(), cmd))
        }
        #[cfg(target_os = "windows")]
        {
            Some(vec!("/c".to_string(), cmd))
        }
    } else {
        None
    };

    // create timestamp
    let now = SystemTime::now();

    // run command
    let mut binding = Command::new(cmd_to_run);
    let cmd = match args {
        Some(s) => binding.args(s),
        None => &mut binding,
    };
    let cmd = cmd.stderr(Stdio::inherit()).stdout(Stdio::inherit());
    let _status = cmd.status().expect("failed to run command!");

    // return time elapsed in milliseconds
    now.elapsed()
        .expect("failed to get elapsed time!")
        .as_millis()
}

fn run_cmd_thread(cmd: String, nullpipe: bool, shell: bool) -> JoinHandle<u128> {
    thread::spawn(move || run_cmd(cmd, nullpipe, shell))
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
        // to keep track of how many iterations actually ran
        let mut ran_iterations = 0;
        while ran_iterations < iterations {
            // to store the JoinHandles of running threads
            let mut handles: Vec<JoinHandle<u128>> = Vec::with_capacity(threads as usize);
            // run threads
            for i in 0..threads {
                println!("Running iteration on thread {}", i + 1);
                // run the command in a thread and store the JoinHandle
                handles.push(run_cmd_thread(cmd.clone(), args.nullpipe, args.shell));
            }
            // loop through every thread's JoinHandle
            // and wait for it to be finished
            for handle in handles {
                total_time += handle.join().unwrap();
                ran_iterations += 1;
            }
        }
        // in case iterations is not perfectly divisibe by threads
        // will cause more iterations to run than asked for
        // so to not break any math adjust iterations
        iterations = ran_iterations;
    }
    // single threading
    else {
        for i in 0..iterations {
            println!("Running iteration {}", i + 1);
            // generate cmd and run command
            total_time += run_cmd(cmd.clone(), args.nullpipe, args.shell)
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
