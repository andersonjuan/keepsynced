/// Configs stores the location and m
use std::path::PathBuf;
use std::time::SystemTime;

pub struct Configs<'a> {
    pub path: &'a PathBuf,
    last_modified: SystemTime,
    configs: &'a PathBuf,
    init: bool,
}

pub fn init<'a>(path: PathBuf, configs: PathBuf) -> Configs<'a> {
    Configs {
        path: &path,
        last_modified: (SystemTime::now()),
        configs: &configs,
        init: true,
    }
}

impl Configs<'_> {
    pub fn update(&self, modified: SystemTime) -> bool {
        if self.init {
            self.last_modified = modified;
            self.init = false;
            return true;
        }

        if update_needed(modified, self.last_modified) {
            self.last_modified = modified;
            self.save_changes();
            return true;
        }
        false
    }

    fn save_changes(&self) {}
}

fn update_needed(new: SystemTime, curr: SystemTime) -> bool {
    match new.duration_since(curr) {
        Ok(_) => {
            return true;
        }
        Err(_) => {
            return false;
        }
    };
}
