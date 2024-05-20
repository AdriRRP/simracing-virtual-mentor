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
    pub mod application {
        pub mod parse_ibt {
            pub mod service;
        }
    }
    pub mod domain {
        pub mod event {
            pub mod ibt_parsed;
        }
        pub mod converter;
    }
    pub mod infrastructure {
        pub mod controller {
            pub mod file;
            pub mod upload_ibt;
        }
        pub mod app_assembler;
        pub mod subscriber {
            pub mod laps {
                pub mod created;
            }
        }
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
    }
    pub mod domain {
        pub mod event {
            pub mod created;
            pub mod deleted;
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

pub mod shared {
    pub mod domain {
        pub mod event;
    }

    pub mod infrastructure {
        pub mod event {
            pub mod dummy;
            pub mod dummy_subscriber;
            pub mod tokio_bus;
        }
    }
}
