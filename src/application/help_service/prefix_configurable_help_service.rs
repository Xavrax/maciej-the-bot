use crate::application::help_service::{HelpLevel, HelpService};
use crate::domain::help::compiled_help_messenger::CompiledHelpMessenger;
use crate::domain::help::HelpMessenger;

pub struct PrefixConfigurableHelpService {
    // todo: consider Cow here
    prefix: String,
    operator_prefix: String,
}

impl PrefixConfigurableHelpService {
    pub fn new(prefix: &str, operator_prefix: &str) -> Self {
        Self {
            prefix: prefix.into(),
            operator_prefix: operator_prefix.into(),
        }
    }
}

impl HelpService for PrefixConfigurableHelpService {
    fn help(&self, help_level: HelpLevel) -> String {
        // todo: make it mockable
        match help_level {
            HelpLevel::User => CompiledHelpMessenger::user(&self.prefix),
            HelpLevel::Operator => {
                CompiledHelpMessenger::admin(&self.prefix, &self.operator_prefix)
            }
        }
        .help_message()
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn should_select_help_based_on_permission_level() {
        let sut = PrefixConfigurableHelpService::new("user", "operator");

        let user_help = sut.help(HelpLevel::User);
        let operator_help = sut.help(HelpLevel::Operator);

        assert_ne!(user_help, operator_help);
    }
}
