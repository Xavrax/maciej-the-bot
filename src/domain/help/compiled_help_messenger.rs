use crate::domain::help::HelpMessenger;

pub struct CompiledHelpMessenger {
    message: String,
}

impl CompiledHelpMessenger {
    pub fn user(prefix: &str) -> Self {
        Self {
            message: include_str!("../../../messages/help.txt").replace("{prefix}", prefix),
        }
    }

    pub fn admin(prefix: &str, operator_prefix: &str) -> Self {
        Self {
            message: include_str!("../../../messages/op_help.txt")
                .replace("{prefix}", &prefix)
                .replace("{op_prefix}", &operator_prefix),
        }
    }
}

impl HelpMessenger for CompiledHelpMessenger {
    fn help_message(self) -> String {
        self.message
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn return_user_help() {
        let prefix = "~";
        let sut = CompiledHelpMessenger::user(prefix);

        let result = sut.help_message();

        assert!(result.contains(prefix));
        assert!(!result.contains("{prefix}"));
    }

    #[test]
    fn return_admin_help() {
        let prefix = "~";
        let operator_prefix = "op";
        let sut = CompiledHelpMessenger::admin(prefix, operator_prefix);

        let result = sut.help_message();

        let merged_prefixes = format!("{}{}", prefix, operator_prefix);
        assert!(result.contains(&merged_prefixes));
        assert!(!result.contains("{prefix}"));
        assert!(!result.contains("{op_prefix}"));
    }
}
