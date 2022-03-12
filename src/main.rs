use anyhow::anyhow;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    io::ErrorKind::InvalidInput,
    net::{SocketAddr, TcpStream},
    process::{Child, Command},
    thread::sleep,
    time::Duration,
};

#[derive(Debug, Deserialize)]
struct Request {
    /// The executable to run
    program: String,

    /// Command line arguments, optional.
    #[serde(default)]
    arguments: Vec<String>,

    /// Environment variables to set, optional.
    #[serde(default)]
    environment: HashMap<String, String>,

    /// TCP ports to poll as an exit condition, optional.
    ///
    /// This is a bit weird. I've found myself wanting to run
    /// a server process for its initialization side effects
    /// (specifically: database migrations). Once the service
    /// initialization is complete, it opens ports to accept
    /// requests. Listing those ports here triggers a service
    /// kill once any of the ports is open.
    #[serde(default)]
    ports: Vec<u16>,
}

fn poll_for_exit_condition(
    mut child: Child,
    program: &str,
    ports: &[u16],
) -> Result<(), anyhow::Error> {
    let addresses: Vec<SocketAddr> = ports
        .iter()
        .map(|port| SocketAddr::from(([127, 0, 0, 1], *port)))
        .collect();

    let one_second = Duration::from_secs(1);

    loop {
        sleep(one_second);

        // If the child has exited, we're done.
        if let Some(status) = child.try_wait()? {
            if status.success() {
                println!("Child {} exited successfully.", program);
                return Ok(());
            } else {
                println!("Child {} failed.", program);
                return Err(anyhow!("{} process failed!", program));
            }
        }

        // If we can connect to one of the ports, we're done: success.
        if let Ok(stream) = TcpStream::connect(&addresses[..]) {
            if let Ok(addr) = stream.peer_addr() {
                println!("Connected to port {}: declaring success.", addr.port());
            } else {
                // This shouldn't happen, but don't worry about it too much.
                println!("Connected to port: declaring success.");
            }
            match child.kill() {
                // `InvalidInput` means the child has already exited
                Err(error) if error.kind() != InvalidInput => {
                    return Err(anyhow!("Failed to kill {}: {}", program, error));
                }
                _ => return Ok(()),
            }
        }
    }
}

fn wait_for_exit(mut child: Child, program: &str) -> Result<(), anyhow::Error> {
    let status = child.wait()?;
    if status.success() {
        println!("Child {} exited successfully.", program);
        Ok(())
    } else {
        println!("Child {} failed.", program);
        Err(anyhow!("{} process failed!", program))
    }
}

fn completion_value(program: &str) -> Result<Value, Error> {
    // TODO: Enhance this to allow the command to return JSON.
    Ok(json!({ program: "success" }))
}

fn run_program(request: &Request) -> Result<Value, Error> {
    println!("Running {} {:?}.", request.program, request.arguments);
    let child = Command::new(&request.program)
        .args(&request.arguments)
        .envs(&request.environment)
        .spawn()?;

    println!("Waiting for {} completion.", &request.program);
    if request.ports.is_empty() {
        wait_for_exit(child, &request.program)?;
    } else {
        poll_for_exit_condition(child, &request.program, &request.ports)?;
    }
    completion_value(&request.program)
}

async fn proxy(event: LambdaEvent<Request>) -> Result<Value, Error> {
    let (request, _context) = event.into_parts();
    run_program(&request)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = service_fn(proxy);
    run(handler).await?;
    Ok(())
}
