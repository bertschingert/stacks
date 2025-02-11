use std::{collections::HashMap, fs, path, rc::Rc};

struct Options {
    include_kernel: bool,
    include_user: bool,
    tasks_of_proc: bool,
    tasks_procs: Vec<usize>,
}

fn usage() {
    eprintln!("Usage: stacks [ku]");
    eprintln!("         k: only include kernel threads");
    eprintln!("         t: only parse threads of given process");
    eprintln!("         u: only include user threads");
}

impl Options {
    fn default_options() -> Options {
        Options {
            include_kernel: true,
            include_user: true,
            tasks_of_proc: false,
            tasks_procs: Vec::new(),
        }
    }

    fn apply(&mut self, opts: Vec<String>) {
        let mut count: usize = 1;
        for s in opts.iter().skip(1) {
            if count == 1 {
                for ch in s.chars() {
                    match ch {
                        'k' => self.include_user = false,
                        't' => {
                            if opts.len() >= 3 {
                                self.tasks_of_proc = true;
                            } else {
                                eprintln!("'t' specified without pid");
                                usage();
                                std::process::exit(1);
                            }
                        }
                        'u' => self.include_kernel = false,
                        _ => {
                            usage();
                            std::process::exit(1);
                        }
                    }
                }
            } else if self.tasks_of_proc {
                let curr_proc: usize = match s.trim().parse() {
                    Ok(num) if num > 0 => num,
                    _ => {
                        eprintln!("invalid pid specified: '{}'", s.trim());
                        usage();
                        std::process::exit(1);
                    }
                };
                if count == 2 {
                    self.tasks_procs = Vec::new();
                }
                self.tasks_procs.push(curr_proc);
            }
            count += 1;
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

fn display_proc_names(procs: &[Rc<ProcEntry>]) -> String {
    let mut pid_hash: HashMap<String, Vec<usize>> = HashMap::new();

    for proc in procs.iter() {
        pid_hash
            .entry(proc.comm.clone())
            .and_modify(|v| v.push(proc.pid))
            .or_insert(vec![proc.pid]);
    }

    let mut names_vec: Vec<_> = pid_hash.iter().collect();
    names_vec.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    let mut s: String = "".to_string();
    for name in names_vec.iter() {
        s.push_str(&format!("({} ", name.0.trim()));
        s.push_str(&format!("{:?}", name.1));
        s.push_str("), ");
    }

    s
}

fn display(p: &HashMap<String, Vec<Rc<ProcEntry>>>) {
    let mut proc_vec: Vec<_> = p.iter().collect();
    proc_vec.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    for (stack, procs) in proc_vec {
        println!("{}", procs.len());
        println!("{}", display_proc_names(procs));
        println!("\n{}", stack);
    }
}

fn process_proc_path(
    options: &Options,
    path: &path::Path,
    hmap: &mut HashMap<String, Vec<Rc<ProcEntry>>>,
) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();

        let proc_ent = match get_proc_ent(&options, &entry.path()) {
            None => continue,
            Some(p) => p,
        };

        let proc_ent = Rc::new(proc_ent);

        hmap.entry(proc_ent.stack.clone())
            .and_modify(|v| v.push(Rc::clone(&proc_ent)))
            .or_insert(vec![Rc::clone(&proc_ent)]);
    }
}

fn main() {
    let mut options = Options::default_options();
    options.apply(std::env::args().collect());

    let mut proc_path_bases: Vec<String> = Vec::new();
    if options.tasks_of_proc {
        for pid in &options.tasks_procs {
            proc_path_bases.push(format!("/proc/{}/task", pid));
        }
    } else {
        proc_path_bases.push(String::from("/proc"));
    }

    let mut proc_hash: HashMap<String, Vec<Rc<ProcEntry>>> = HashMap::new();

    for path in proc_path_bases {
        let proc_path = path::Path::new(&path);
        process_proc_path(&options, &proc_path, &mut proc_hash);
    }

    display(&proc_hash);
}
