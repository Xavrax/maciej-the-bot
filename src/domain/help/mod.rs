pub mod compiled_help_messenger;

pub trait HelpMessenger {
    fn help_message(self) -> String;
}
