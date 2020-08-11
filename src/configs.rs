use std::time::{Duration, SystemTime};

/// Configs stores the location and m
mod configs {
    pub struct Configs {
        pub path: td::path::PathBuf,
        last_modified: SystemTime,
        configs: td::path::PathBuf,
        init: Boolean,
    }

    impl Configs {
        pub fn init(path: td::path::PathBuf, configs: td::path::PathBuf) {
            Configs {
                path: path,
                last_modified: (SystemTime::now()),
                configs: config,
                init: true,
            }
        }

        pub fn update(&self, modified: SystemTime) -> Boolean  {
            if (self.init) {
                self.last_modified = modified;
                self.init = false;
                return true;
            }

            if (update_needed(modified, self.last_modified)) {
                self.last_modified = modified;
                self.save_changes()
                return true;
            }
            false;
        }

        fn save_changes(&self) {

        }
    }
    
    fn update_needed(new: SystemTime, curr: SystemTime) -> Boolean {
        match (new.duration_since(curr)) {
            Ok(_) => { true },
            Err(_) => { false },
        };
    }
}
