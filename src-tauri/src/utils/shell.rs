use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    thread,
};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdoutPayload {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StderrPayload {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPayload {
    pub status: bool,
}

// Use this if want get the output only, like --dump-json
pub fn run_command(program: String, args: Vec<&str>) -> Result<String, String> {
    // println!("Program: {}\nArgs: {:?}", program, args);

    let mut output = Command::new(program);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        output.creation_flags(0x08000000); // CREATE_NO_WINDOW constant
    }

    output.args(args);

    let output = output.output();

    match output {
        Ok(value) => Ok(String::from_utf8_lossy(&value.stdout).to_string()),
        Err(error) => Err(error.to_string()),
    }
}

// Use this for downloading (no need of output)
pub fn process_command(app: AppHandle, program: String, args: Vec<String>) {
    // println!("Program: {}\nArgs: {:?}", program, args);

    let mut child = Command::new(program);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        child.creation_flags(0x08000000); // CREATE_NO_WINDOW constant
    }

    child
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = child.spawn().expect("program cannot run");

    let stdout = child.stdout.take().expect("failed to get stdout");
    let stderr = child.stderr.take().expect("failed to get stderr");

    let stdout_app = app.clone();
    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            // println!("stdout: {}", line.unwrap());
            let _ = stdout_app.emit(
                "stdoutCommand",
                StdoutPayload {
                    message: line.unwrap(),
                },
            );
        }
    });

    let stderr_app = app.clone();
    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            // println!("stderr: {}", line.unwrap());
            let _ = stderr_app.emit(
                "stderrCommand",
                StderrPayload {
                    message: line.unwrap(),
                },
            );
        }
    });

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    let status = child.wait().expect("error waiting child");
    let _ = app.emit(
        "statusCommand",
        StatusPayload {
            status: status.success(),
        },
    );

    // String::from_utf8_lossy(&child.stdout).to_string()
    // "DEBUG!".to_string()
}
