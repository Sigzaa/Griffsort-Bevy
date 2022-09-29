use bevy_console::ConsoleCommand;

/// Example command
#[derive(ConsoleCommand)]
#[console_command(name = "example")]
pub struct ExampleCommand {
    /// Some message
    msg: String,
}

pub fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
    if let Some(ExampleCommand { msg }) = log.take()
    {
        // handle command
    }
}
