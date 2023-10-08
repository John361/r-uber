use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UberInput {
    pub path: String
}

impl UberInput {

    pub fn is_same_path(&self, other_path: &str) -> bool {
        other_path.contains(&self.path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_instance() -> UberInput {
        UberInput {
            path: "some/relative/path".to_string()
        }
    }

    #[test]
    fn should_same_path_true() {
        let content: UberInput = simple_instance();
        let result: bool = content.is_same_path("add/absolute/path/to/some/relative/path");

        assert_eq!(result, true);
    }

    #[test]
    fn should_same_path_false() {
        let content: UberInput = simple_instance();
        let result: bool = content.is_same_path("add/absolute/path/to/relative/path");

        assert_eq!(result, false);
    }
}
