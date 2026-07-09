//! Demo: Interactive Shell with Filesystem
//!
//! This demo shows how to use the shell, filesystem, and GUI modules together
//! to create an interactive user interface.

#[cfg(test)]
mod tests {
    use crate::shell::{Shell, Command};
    use crate::fs::FileType;

    #[test]
    fn test_shell_creation() {
        let shell = Shell::new();
        assert_eq!(shell.current_user, "root");
        assert_eq!(shell.current_dir, "/");
    }

    #[test]
    fn test_command_parsing() {
        let mut shell = Shell::new();
        let cmd = shell.parse_command("ls /home");
        assert_eq!(cmd.name, "ls");
        assert_eq!(cmd.args.len(), 1);
    }

    #[test]
    fn test_cd_command() {
        let mut shell = Shell::new();
        shell.change_directory("home");
        assert_eq!(shell.current_dir, "/home");
    }

    #[test]
    fn test_directory_creation() {
        let mut vfs = crate::fs::vfs::VFS::new();
        let inode = vfs.create_file("test_dir", FileType::Directory);
        assert_eq!(inode, 0);
    }
}
