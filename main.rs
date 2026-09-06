use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::io::{self, BufRead, Write};

// PID Namespace Simulator
// Each namespace gets its own PID counter starting at 1.
// Commands:
//   NEWNS  -> allocate next ns id, init empty process table, print id
//   FORK <ns> <name>  -> spawn process; assign in-ns pid; print pid
//   EXIT <ns> <pid>  -> mark exited; print OK
//   PS <ns>  -> print '<pid> <name> <state>' sorted by pid

enum ProcessState {
    RUNNING,
    EXITED,
}

impl Display for ProcessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessState::RUNNING => write!(f, "running"),
            ProcessState::EXITED => write!(f, "exited"),
        }
    }
}

struct Process {
    name: String,
    state: ProcessState,
}

struct Namespace {
    next_process_id: u32,
    processes: BTreeMap<u32, Process>,
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let mut buf: Vec<String> = Vec::new();

    let mut next_ns_id: u32 = 1;
    let mut namespaces: HashMap<u32, Namespace> = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.trim_end_matches('\n');
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "NEWNS" => {
                namespaces.insert(
                    next_ns_id,
                    Namespace {
                        next_process_id: 1,
                        processes: BTreeMap::new(),
                    },
                );
                println!("{}", next_ns_id);
                next_ns_id += 1;
            }
            "FORK" => {
                let namespace_id = parts[1].parse::<u32>().unwrap();
                let namespace = namespaces.get_mut(&namespace_id).unwrap();
                namespace.processes.insert(
                    namespace.next_process_id,
                    Process {
                        name: parts[2].into(),
                        state: ProcessState::RUNNING,
                    },
                );
                println!("{}", namespace.next_process_id);
                namespace.next_process_id += 1;
            }
            "EXIT" => {
                let namespace_id = parts[1].parse::<u32>().unwrap();
                let process_id = parts[2].parse::<u32>().unwrap();
                let process = namespaces
                    .get_mut(&namespace_id)
                    .unwrap()
                    .processes
                    .get_mut(&process_id)
                    .unwrap();
                process.state = ProcessState::EXITED;
                println!("OK");
            }
            "PS" => {
                let namespace_id = parts[1].parse::<u32>().unwrap();
                namespaces[&namespace_id].processes.iter().for_each(|p| {
                    println!("{} {} {}", p.0, p.1.name, p.1.state);
                });
            }
            _ => {}
        }
    }
    writeln!(out, "{}", buf.join("\n")).unwrap();
}
