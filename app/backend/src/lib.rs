pub mod analysis {
    pub mod application {
        pub mod create {
            pub mod service;
        }
        pub mod delete {
            pub mod service;
        }
        pub mod find {
            pub mod by_id {
                pub mod service;
            }
            pub mod by_criteria {
                pub mod service;
            }
        }
    }
    pub mod domain {
        pub mod analyses;
        pub mod analysis;
        pub mod repository;
    }
    pub mod infrastructure {
        pub mod repository {
            pub mod in_memory;
        }
    }
}

pub mod api {
    pub mod infrastructure {
        pub mod app_assembler;
        pub mod controller {
            pub mod file;
            pub mod ibt_extractor;
            pub mod lap;
        }
        pub mod subscriber {
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

pub mod lap {
    pub mod application {
        pub mod create {
            pub mod service;
        }
        pub mod delete {
            pub mod service;
        }
        pub mod find {
            pub mod by_id {
                pub mod service;
            }
            pub mod by_criteria {
                pub mod service;
            }
            pub mod header_by_id {
                pub mod service;
            }
            pub mod headers_by_criteria {
                pub mod service;
            }
        }
    }
    pub mod domain {
        pub mod lap;
        pub mod laps;
        pub mod repository;
    }
    pub mod infrastructure {
        pub mod repository {
            pub mod in_memory;
        }
    }
}

pub mod file {
    pub mod application {
        pub mod create {
            pub mod service;
        }
        pub mod delete {
            pub mod service;
        }
        pub mod find {
            pub mod by_id {
                pub mod service;
            }
            pub mod by_criteria {
                pub mod service;
            }
        }
        pub mod validate {
            pub mod service;
        }
    }
    pub mod domain {
        pub mod event {
            pub mod created;
            pub mod deleted;
            pub mod validated;
        }
        pub mod file;
        pub mod files;
        pub mod repository;
    }
    pub mod infrastructure {
        pub mod repository {
            pub mod in_memory;
        }
    }
}

pub mod ibt {
    pub mod domain {
        pub mod file;
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

pub mod shared {
    pub mod domain {
        pub mod event;
    }

    pub mod infrastructure {
        pub mod event {
            pub mod tokio_bus;
        }
    }
}
