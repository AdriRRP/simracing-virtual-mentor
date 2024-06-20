extern crate symracing_virtual_mentor_shared as shared;
pub mod api {
    pub mod infrastructure {
        pub mod app_assembler;
        pub mod controller {
            pub mod analysis;
            pub mod file;
            pub mod ibt_extractor;
            pub mod lap;
        }
        pub mod event {
            pub mod tokio_bus;
        }
        pub mod repository {
            pub mod mongo_analysis;
            pub mod mongo_file;
            pub mod mongo_lap;
        }
        pub mod subscriber {
            pub mod on_analysis_created {
                pub mod do_analysis;
            }
            pub mod on_file_deleted {
                pub mod delete_laps;
            }
            pub mod on_ibt_extracted {
                pub mod validate_file;
            }
            pub mod manager;
        }
        pub mod settings;
    }
}

pub mod ibt_extractor {
    pub mod application {
        pub mod extract {
            pub mod service;
        }
    }
    pub mod domain {
        pub mod event {
            pub mod extracted;
        }
        pub mod converter;
    }
}
