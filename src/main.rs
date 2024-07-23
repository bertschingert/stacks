use std::{collections::HashMap, fs, path, rc::Rc};

struct Options {
    include_kernel: bool,
    include_user: bool,
}

fn usage() {
    eprintln!("Usage: stacks [ku]");
    eprintln!("         k: only include kernel threads");
    eprintln!("         u: only include user threads");
}

impl Options {
    fn default_options() -> Options {
        Options {
            include_kernel: true,
            include_user: true,
        }
    }

    fn apply(&mut self, opts: Vec<String>) {
        for s in opts.iter().skip(1) {
            for ch in s.chars() {
                match ch {
                    'k' => self.include_user = false,
                    'u' => self.include_kernel = false,
                    _ => {
                        usage();
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}

struct ProcStat {
    ppid: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseProcStatError;

impl std::str::FromStr for ProcStat {
    type Err = ParseProcStatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // fields we care about start after final ')', which terminates comm name
        let rest = s.split(')').last().ok_or(ParseProcStatError)?;

        let mut fields = rest.split(' ');
        let ppid: usize = fields
            .nth(2)
            .and_then(|s| s.parse().ok())
            .ok_or(ParseProcStatError)?;

        Ok(ProcStat { ppid })
    }
}

struct ProcEntry {
    pid: usize,
    comm: String,
    stack: String,
}

fn read_proc_data(path: &path::Path, file: &str) -> Result<String, std::io::Error> {
    // TODO: switch to inspect_err() once on a new enough rustc version
    fs::read_to_string(path.join(file)).map_err(|e| {
        eprintln!("Could not read {}: {}", path.join(file).display(), e);
        e
    })
}

fn get_proc_ent(options: &Options, path: &path::Path) -> Option<ProcEntry> {
    let pid: usize = match path.file_name().unwrap().to_str().unwrap().parse() {
        Err(_) => return None,
        Ok(p) => p,
    };

    let stat: ProcStat = read_proc_data(path, "stat").ok()?.parse().ok()?;

    if stat.ppid == 2 || pid == 2 {
        if !options.include_kernel {
            return None;
        }
    } else if !options.include_user {
        return None;
    }

    let comm = read_proc_data(path, "comm").ok()?;

    let stack = read_proc_data(path, "stack").ok()?;

    Some(ProcEntry { pid, comm, stack })
}

fn main() {
    let proc_path = path::Path::new("/proc");

    let mut proc_hash: HashMap<String, Vec<Rc<ProcEntry>>> = HashMap::new();

    let mut options = Options::default_options();
    options.apply(std::env::args().collect());

    for entry in fs::read_dir(proc_path).unwrap() {
        let entry = entry.unwrap();

        let proc_ent = match get_proc_ent(&options, &entry.path()) {
            None => continue,
            Some(p) => p,
        };

        let proc_ent = Rc::new(proc_ent);

        proc_hash
            .entry(proc_ent.stack.clone())
            .and_modify(|v| v.push(Rc::clone(&proc_ent)))
            .or_insert(vec![Rc::clone(&proc_ent)]);
    }

    let mut proc_vec: Vec<_> = proc_hash.iter().collect();
    proc_vec.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    for (stack, procs) in proc_vec {
        println!("{}", procs.len());
        for p in procs {
            print!("{} ", p.comm.trim());
        }
        println!("\n{}", stack);
    }
}
