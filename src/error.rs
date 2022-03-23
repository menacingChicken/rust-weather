#[derive(Debug)]
pub struct MyError {
    msg: String
}

impl std::fmt::Display for MyError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(fmt, "{}", self.msg)
    }
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl MyError {
    pub fn new(msg: &str) -> Box<dyn std::error::Error>
    {
        Box::new(MyError {
            msg: msg.to_owned()
        })
    }
}