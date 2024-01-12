use std::{
    env,
    fs::File,
    io::Write,
    process::{self, Command, Stdio},
    time::{Duration, Instant},
};

use rand::{seq::SliceRandom, Rng};

fn main() {
    env::set_current_dir("/home/grayshade/Projects/georust/gdal").unwrap();
    let binary = "target/release/deps/gdal-a5f1a0f14e3eca4c";
    let mut tests = "spatial_ref::transform_opts::tests::invalid_transformation spatial_ref::transform_opts::tests::set_coordinate_operation vector::layer::tests::test_create_layer_options vector::layer::tests::test_field_in_layer".split(' ').collect::<Vec<_>>();
    let pid = process::id();
    let mut log_file = File::create(format!("log_{pid}.txt")).unwrap();

    let mut duration = 5;
    let mut rng = rand::thread_rng();
    loop {
        tests.shuffle(&mut rng);
        let mut i = 0;
        while i < tests.len() {
            let excluded = tests[i];
            println!("Excluding {}", excluded);
            let now = Instant::now();
            let mut crashed = false;
            while (Instant::now() - now) < Duration::from_secs(duration) {
                let mut command = Command::new(binary);
                command.stdout(Stdio::null());
                command.stderr(Stdio::null());
                command.arg("-q");
                let threads = rng.gen_range(2..4);
                command.arg("--test-threads");
                command.arg(threads.to_string());
                for j in 0..tests.len() {
                    if j != i {
                        command.arg(tests[j]);
                    }
                }
                let status = command.status().unwrap();
                if !status.success() {
                    crashed = true;
                    println!("Crashed {:?}: {:?}", command, status);
                    break;
                }
            }
            if crashed {
                println!("Dropping {}", excluded);
                tests.remove(i);
                writeln!(&mut log_file, "{}", excluded).unwrap();
            } else {
                i += 1;
            }
        }
        duration += 1;
    }
}
