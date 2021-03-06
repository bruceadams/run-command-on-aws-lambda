use anyhow::anyhow;
use async_process::{Child, Command};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    io::ErrorKind::InvalidInput,
    net::{SocketAddr, TcpStream},
    time::Duration,
};
use tokio::time::sleep;

#[derive(Debug, Default, Deserialize)]
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

async fn poll_for_exit_condition(
    mut child: Child,
    program: &str,
    ports: &[u16],
) -> Result<(), anyhow::Error> {
    let addresses: Vec<SocketAddr> = ports
        .iter()
        .map(|port| SocketAddr::from(([127, 0, 0, 1], *port)))
        .collect();

    let tenth_of_a_second = Duration::from_millis(100);

    loop {
        sleep(tenth_of_a_second).await;

        // If the child has exited, we're done.
        if let Some(status) = child.try_status()? {
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

async fn wait_for_exit(mut child: Child, program: &str) -> Result<(), anyhow::Error> {
    let status = child.status().await?;
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

async fn run_program(request: &Request) -> Result<Value, Error> {
    println!("Running {} {:?}.", request.program, request.arguments);
    let child = Command::new(&request.program)
        .args(&request.arguments)
        .envs(&request.environment)
        .spawn()?;

    println!("Waiting for {} completion.", &request.program);
    if request.ports.is_empty() {
        wait_for_exit(child, &request.program).await?;
    } else {
        poll_for_exit_condition(child, &request.program, &request.ports).await?;
    }
    completion_value(&request.program)
}

async fn proxy(event: LambdaEvent<Request>) -> Result<Value, Error> {
    let (request, _context) = event.into_parts();
    run_program(&request).await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = service_fn(proxy);
    run(handler).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::consts::OS;
    use tokio;

    #[tokio::test]
    async fn simple_success() {
        let request = Request {
            program: "true".to_string(),
            arguments: vec![],
            environment: HashMap::new(),
            ports: vec![],
        };
        let result = run_program(&request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn simple_failure() {
        let request = Request {
            program: "false".to_string(),
            arguments: vec![],
            environment: HashMap::new(),
            ports: vec![],
        };
        let result = run_program(&request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn polled_success() {
        let request = Request {
            program: "true".to_string(),
            arguments: vec![],
            environment: HashMap::new(),
            ports: vec![8080],
        };
        let result = run_program(&request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn polled_failure() {
        let request = Request {
            program: "false".to_string(),
            arguments: vec![],
            environment: HashMap::new(),
            ports: vec![8080],
        };
        let result = run_program(&request).await;
        assert!(result.is_err());
    }

    /// This test is a _nice to have_, but won't run everywhere.
    /// It uses `nc`, aka netcat, which has different, incompatible variants.
    #[tokio::test]
    async fn exit_on_open_port() {
        let arguments = if OS == "linux" {
            // This is netcat-traditional, bundled in alpine, our CI platform.
            vec!["-lp".to_string(), "8080".to_string()]
        } else {
            // This is netcat-openbsd, bundled on macOS, my main dev platform.
            vec!["-l".to_string(), "8080".to_string()]
        };
        let request = Request {
            program: "nc".to_string(),
            arguments,
            ports: vec![8080],
            ..Default::default()
        };
        let result = run_program(&request).await;
        assert!(result.is_ok());
    }
}
