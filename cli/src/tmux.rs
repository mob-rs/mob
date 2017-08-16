use errors::Result;
use std::process::Command;
use errors::ResultExt;

pub struct Window {
    pub index: i32,
    pub session: String,
}

impl Window {
    pub fn from_string(raw: &str) -> Result<Window> {
        let split = raw.split(":").collect::<Vec<&str>>();
        let session = split.first().chain_err(|| "Missing session")?;
        let index = split[1]
            .split(".")
            .collect::<Vec<&str>>()
            .first()
            .chain_err(|| "Missing index")
            .and_then(|index| {
                index.parse::<i32>().chain_err(|| "Failed to parse index to int")
            })?;

        Ok(Window::new(index, session.clone()))
    }

    fn new(index: i32, session: &str) -> Window {
        Window {
            index: index,
            session: session.into(),
        }
    }
}

pub fn new_window_with_command(command: &str) -> Result<Window> {
    let output = Command::new("tmux")
        .arg("new-window")
        .arg("-P")
        .arg(command)
        .output()?;

    let raw = String::from_utf8_lossy(&output.stdout);

    Window::from_string(&raw)
}

#[cfg(test)]
mod test {
    use super::Window;

    #[test]
    fn test_window_from_string() {
        let raw = "mob:4.1";

        let window = Window::from_string(raw).unwrap();

        assert_eq!(window.index, 4);
        assert_eq!(window.session, "mob".to_owned());
    }
}
