use std::{fs, io::Write, path::Path, process::Command};

const BASE_ADDRESS: u32 = 0x80400000;
const STEP: u32 = 0x20000;
const LINKER: &'static str = "src/linker.ld";

fn main() {
    let mut app_id = 0;

    let mut apps: Vec<_> = fs::read_dir("src/bin")
        .expect("Failed to read the directory")
        .map(|x| {
            x.expect("Failed to open the item")
                .file_name()
                .into_string()
                .expect("Failed to convert")
        })
        .collect();

    apps.sort();

    for app in &apps {
        let app = app.trim_end_matches('.').to_string();

        let file_content = fs::read_to_string(LINKER).expect("Failed to read the file");
        let lines_before: Vec<_> = file_content.lines().collect();

        let lines: Vec<_> = file_content
            .lines()
            .map(|line| {
                line.replace(
                    &format!("{:x}", BASE_ADDRESS),
                    &format!("{:x}", BASE_ADDRESS + STEP * app_id as u32),
                )
            })
            .collect();

        let file = Path::new(LINKER);
        let mut f = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file)
            .expect("Failed to open linker file");
        write!(f, "{}", lines.join("\n")).expect("Failed to write to the file");

        let mut cargo = Command::new("cargo");
        cargo
            .arg("build")
            .arg("--bin")
            .arg(&app)
            .arg("--release")
            .output()
            .expect("Failed to execute command");

        println!(
            "[build.py] application {} start with address {}",
            &app,
            format!("{:x}", BASE_ADDRESS + STEP * app_id)
        );

        let mut f = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file)
            .expect("Failed to open linker file");
        write!(f, "{}", lines_before.join("\n")).unwrap();

        app_id += 1;
    }
}
