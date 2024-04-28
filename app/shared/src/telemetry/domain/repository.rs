use crate::telemetry::domain::session::Session;

pub trait Repository {
    fn find_all(self) -> Vec<Session>;
    fn find_by_id(self, id: String) -> Option<Session>;
}