use crate::ibt::domain::file::var_header::VarHeader;

pub struct VarFilter {
    allowed_names: Vec<String>,
}

impl VarFilter {
    pub fn new(allowed_names: Vec<String>) -> Self {
        Self { allowed_names }
    }

    pub fn allow(&self, var_header: &VarHeader) -> bool {
        self.allowed_names.iter().any(|a| {
            var_header
                .name()
                .to_lowercase()
                .trim()
                .contains(a.to_lowercase().trim())
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ibt::domain::file::var_header::var_type::VarType;

    fn test_var_header() -> VarHeader {
        VarHeader {
            var_type: VarType::Char,
            offset: 0,
            count: 0,
            count_as_time: 0,
            name: [
                'S', 'e', 's', 's', 'i', 'o', 'n', 'T', 'i', 'm', 'e', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0',
            ],
            description: [
                'S', 'e', 'c', 'o', 'n', 'd', 's', ' ', 's', 'i', 'n', 'c', 'e', ' ', 's', 'e',
                's', 's', 'i', 'o', 'n', ' ', 's', 't', 'a', 'r', 't', '.', '.', '.', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0',
            ],
            unit: [
                's', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',
                '\0', '\0', '\0', '\0',
            ],
        }
    }

    #[test]
    fn filter_allow_header() {
        let filter = VarFilter::new(vec!["session".to_string()]);
        assert!(filter.allow(&test_var_header()))
    }

    #[test]
    fn filter_trimmed_allow_header() {
        let filter = VarFilter::new(vec!["    session     ".to_string()]);
        assert!(filter.allow(&test_var_header()))
    }

    #[test]
    fn filter_reject_header() {
        let filter = VarFilter::new(vec!["not in header".to_string()]);
        assert!(!filter.allow(&test_var_header()))
    }
}
