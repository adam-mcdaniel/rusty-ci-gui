use std::fmt::{Display, Formatter, Error};


pub struct MasterYaml {
    master: Master,
    merge_handler: MergeHandler,
    workers: Vec<Worker>,
    schedulers: Vec<Scheduler>,
    builders: Vec<Builder>
}

impl MasterYaml {
    pub fn new(master: Master, merge_handler: MergeHandler,
               workers: Vec<Worker>, schedulers: Vec<Scheduler>,
               builders: Vec<Builder>) -> Self {
        Self {
            master,
            merge_handler,
            workers,
            schedulers,
            builders
        }
    }
}

impl Display for MasterYaml {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut workers = String::from("workers:\n");
        for worker in &self.workers {
            workers += &worker.to_string();
        }

        let mut schedulers = String::from("schedulers:\n");
        for scheduler in &self.schedulers {
            schedulers += &scheduler.to_string();
        }

        let mut builders = String::from("builders:\n");
        for builder in &self.builders {
            builders += &builder.to_string();
        }

        write!(f, r#"{}
{}
{}
{}
{}"#, self.master, self.merge_handler, workers,
    schedulers, builders)
    }
}


impl Default for MasterYaml {
    fn default() -> Self {
        Self::new(
            Master::default(),
            MergeHandler::default(),
            vec![Worker::default()],
            vec![Scheduler::default()],
            vec![Builder::default()]
        )
    }
}


/// Represents the data that is stored in the master
/// section of the master yaml file
pub struct Master {
    title: String,
    title_url: String,
    webserver_ip: String,
    repo: String,
    poll_interval: i32,
}

#[allow(dead_code)]
impl Master {
    pub fn new(title: String, title_url: String,
               webserver_ip: String, repo: String,
               poll_interval: i32) -> Self {
        Master {title, title_url, webserver_ip, repo, poll_interval}
    }


    pub fn set_repo(&mut self, repo: String) { self.repo = repo }
    pub fn set_title(&mut self, title: String) { self.title = title }
    pub fn set_title_url(&mut self, title_url: String) { self.title_url = title_url }
    pub fn set_webserver_ip(&mut self, webserver_ip: String) { self.webserver_ip = webserver_ip }
    pub fn set_poll_interval(&mut self, poll_interval: i32) { self.poll_interval = poll_interval }
}

impl Default for Master {
    fn default() -> Self {
        Self::new(
            String::from("Rusty-CI"),
            String::from("https://github.com/adam-mcdaniel/rusty-ci"),
            String::from("localhost"),
            String::from("https://github.com/adam-mcdaniel/rusty-ci"),
            120
        )
    }
}

impl Display for Master {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, r#"master:
  title: "{}"
  title-url: "{}"
  webserver-ip: {}
  repo: "{}"
  poll-interval: {}
"#, self.title, self.title_url, self.webserver_ip,
    self.repo, self.poll_interval)
    }
}


/// Represents the version control system that the
/// repo is controlled by.
pub enum VCS {
    GitHub,
    GitLab,
    Unsupported
}


impl From<String> for VCS {
    fn from(s: String) -> Self {
        match s.as_str() {
            "github" => VCS::GitHub,
            "gitlab" => VCS::GitLab,
            otherwise => {
                println!("Unsupported VCS: {}!", otherwise);
                VCS::Unsupported
            }
        }
    }
}

impl Display for VCS {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", match self {
            VCS::GitHub => "github",
            VCS::GitLab => "gitlab",
            VCS::Unsupported => "unsupported",
        })
    }
}


/// Represents the merge-request-handler section
/// in the master yaml file.
pub struct MergeHandler {
    vcs: VCS,
    owner: String,
    repo_name: String,
    whitelist: Vec<String>
}

impl MergeHandler {
    fn new(vcs: VCS, owner: String, repo_name: String, whitelist: Vec<String>) -> Self {
        Self {
            vcs, owner, repo_name, whitelist
        }
    }
}

impl Default for MergeHandler {
    fn default() -> Self {
        Self::new(
            VCS::GitHub,
            String::from("adam-mcdaniel"),
            String::from("rusty-ci"),
            vec![String::from("adam-mcdaniel")]
        )
    }
}

impl Display for MergeHandler {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let whitelist = String::from("   - ") + &self.whitelist.join("\n   - ");

        write!(f, r#"merge-request-handler:
  version-control-system: {}
  owner: {}
  repo-name: {}
  whitelist:
{}
"#, self.vcs, self.owner, self.repo_name, whitelist)
    }
}

/// Represents an individual worker inside the workers
/// section in the master yaml.
pub struct Worker {
    name: String,
    master_ip: String,
    master_port: i32,
    base_dir: String,
    password: String
}

impl Worker {
    fn new(name: String, master_ip: String, master_port: i32, base_dir: String, password: String) -> Self {
        Self {
            name,
            master_ip,
            master_port,
            base_dir,
            password
        }
    }
}

impl Default for Worker {
    fn default() -> Self {
        Self::new(
            String::from("test-worker"),
            String::from("localhost"),
            9989,
            String::from("/home/adam/Desktop/rusty-ci/testing/test-worker"),
            String::from("pass"),
        )
    }
}

impl Display for Worker {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, r#"  {}:
    masterhost: {}
    masterport: {}
    basedir: '{}'
    password: {}

"#, self.name, self.master_ip, 
    self.master_port, self.base_dir,
    self.password)

    }
}


/// Represents an individual scheduler inside the schedulers
/// section in the master yaml.
pub struct Scheduler {
    name: String,
    builders: Vec<String>,
    branch: String,
    triggers: Vec<String>,
    password: String
}

impl Scheduler {
    fn new(name: String, builders: Vec<String>, branch: String, triggers: Vec<String>, password: String) -> Self {
        Self {
            name,
            builders,
            branch,
            triggers,
            password
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new(
            String::from("ci-change"),
            vec![String::from("rusty-ci-test")],
            String::from(".*"),
            vec![
                String::from(".*\\.yaml"),
                String::from(".*\\.sh"),
                String::from(".*Makefile"),
            ],
            String::from("ok to test"),
        )
    }
}

impl Display for Scheduler {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let builders = String::from("\n      - ") + &self.builders.join("\n      - ");
        let triggers = String::from("\n      - ") + &self.triggers.iter().map(|s| String::from("\"") + s + "\"").collect::<Vec<String>>().join("\n      - ");
        
        write!(f, r#"  {}:
    builders:{}

    branch: "{}"
    triggers: {}
    password: {}

"#, self.name, builders, 
    self.branch, triggers,
    self.password)

    }
}


/// Represents an individual builder inside the builders
/// section in the master yaml file.
pub struct Builder {
    name: String,
    script: Vec<String>,
    workers: Vec<String>,
    repo: String
}

impl Builder {
    fn new(name: String, script: Vec<String>, workers: Vec<String>, repo: String) -> Self {
        Self {
            name,
            script,
            workers,
            repo
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new(
            String::from("rusty-ci-test"),
            vec![
                String::from("echo Hello world!"),
                String::from("echo Im an instruction in a script!")
            ],
            vec![
                String::from("test-worker"),
            ],
            String::from("https://github.com/adam-mcdaniel/rusty-ci"),
        )
    }
}

impl Display for Builder {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let workers = String::from("\n      - ") + &self.workers.join("\n      - ");
        let script = String::from("\n      - ") + &self.script.join("\n      - ");
        
        write!(f, r#"  {}:
    workers: {}
    script: {}
    repo: {}
"#, self.name,
    workers, 
    script,
    self.repo)

    }
}