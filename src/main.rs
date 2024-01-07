use std::{
    fs,
    path,
    collections::HashMap,
    rc::Rc,
};

struct ProcEntry {
    _pid: usize,
    comm: String,
    stack: String,
}

fn get_proc_ent(path: &path::Path) -> Option<ProcEntry> {
    let _pid = match path.file_name().unwrap()
        .to_str().unwrap()
        .parse::<usize>() {
        Err(_) => return None,
        Ok(p) => p,
    };

    let comm = match fs::read_to_string(path.join("comm")) {
        Err(e) => {
            eprintln!("could not read {}: {}", path.join("comm").display(), e);
            return None;
        },
        Ok(mut s) => {
            s.pop(); /* trim newline from comm */
            s
        }
    };

    let stack = match fs::read_to_string(path.join("stack")) {
        Err(e) => {
            eprintln!("could not read {}: {}", path.join("stack").display(), e);
            return None;
        },
        Ok(s) => s,
    };

    Some(ProcEntry { _pid, comm, stack, })
}

fn main() {
    let proc_path = path::Path::new("/proc");

    let mut proc_hash: HashMap<String, Vec<Rc<ProcEntry>>> = HashMap::new();

    for entry in fs::read_dir(proc_path).unwrap() {
        let entry = entry.unwrap();

        let proc_ent = match get_proc_ent(&entry.path()) {
            None => continue,
            Some(s) => p,
        };

        let proc_ent = Rc::new(proc_ent);

        proc_hash.entry(proc_ent.stack.clone())
            .and_modify(|v| v.push(proc_ent.clone()))
            .or_insert(vec![proc_ent.clone()]);
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
