/// Built-in shell commands
use alloc::string::{String, ToString};
use super::{Command, CommandResult, Shell};
use crate::vga::writer::{VgaWriter, Color};

pub fn execute(cmd: Command, shell: &mut Shell) -> CommandResult {
    match cmd.name.as_str() {
        "ls" => cmd_ls(&cmd.args, shell),
        "cd" => cmd_cd(&cmd.args, shell),
        "pwd" => cmd_pwd(shell),
        "echo" => cmd_echo(&cmd.args),
        "help" => cmd_help(),
        "clear" => cmd_clear(),
        "uname" => cmd_uname(),
        "whoami" => cmd_whoami(shell),
        "exit" | "quit" => CommandResult::Exit,
        "" => CommandResult::Success(String::new()),
        _ => CommandResult::Error(format!("Unknown command: {}", cmd.name)),
    }
}

fn cmd_ls(_args: &[String], shell: &Shell) -> CommandResult {
    let mut vga = VgaWriter::new();
    vga.set_color(Color::Cyan, Color::Black);
    vga.write_str(".");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("   ");
    vga.set_color(Color::Cyan, Color::Black);
    vga.write_str("..");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("   ");
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("bin");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("   ");
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("etc");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("   ");
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("home");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("\n");
    CommandResult::Success(String::from("Directory listing displayed"))
}

fn cmd_cd(args: &[String], shell: &mut Shell) -> CommandResult {
    if args.is_empty() {
        shell.change_directory("/home");
        return CommandResult::Success(String::new());
    }
    
    shell.change_directory(&args[0]);
    CommandResult::Success(String::new())
}

fn cmd_pwd(shell: &Shell) -> CommandResult {
    let mut vga = VgaWriter::new();
    vga.write_str(&shell.current_dir);
    vga.write_str("\n");
    CommandResult::Success(shell.current_dir.clone())
}

fn cmd_echo(args: &[String]) -> CommandResult {
    let mut vga = VgaWriter::new();
    for (i, arg) in args.iter().enumerate() {
        if i > 0 {
            vga.write_str(" ");
        }
        vga.write_str(arg);
    }
    vga.write_str("\n");
    CommandResult::Success(args.join(" "))
}

fn cmd_help() -> CommandResult {
    let mut vga = VgaWriter::new();
    vga.set_color(Color::Yellow, Color::Black);
    vga.write_str("Available Commands:\n");
    vga.set_color(Color::White, Color::Black);
    vga.write_str("  ls      - List directory contents\n");
    vga.write_str("  cd      - Change directory\n");
    vga.write_str("  pwd     - Print working directory\n");
    vga.write_str("  echo    - Display text\n");
    vga.write_str("  uname   - Print system information\n");
    vga.write_str("  whoami  - Display current user\n");
    vga.write_str("  clear   - Clear screen\n");
    vga.write_str("  help    - Show this help\n");
    vga.write_str("  exit    - Exit shell\n");
    CommandResult::Success(String::from("Help displayed"))
}

fn cmd_clear() -> CommandResult {
    let mut vga = VgaWriter::new();
    vga.clear_screen();
    CommandResult::Success(String::new())
}

fn cmd_uname() -> CommandResult {
    let mut vga = VgaWriter::new();
    vga.write_str("Comet OS 0.1.0 (x86_64)\n");
    CommandResult::Success(String::from("Comet OS 0.1.0 (x86_64)"))
}

fn cmd_whoami(shell: &Shell) -> CommandResult {
    let mut vga = VgaWriter::new();
    vga.write_str(&shell.current_user);
    vga.write_str("\n");
    CommandResult::Success(shell.current_user.clone())
}
