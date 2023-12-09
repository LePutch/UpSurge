#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    SSH,
    //GRPC,
}
#[derive(Debug)]
pub enum Method {
    THREADED,
    ASYNC,
}
#[derive(Debug)]
pub enum Deployment {
    NODEPLOY,

    AUTODEPLOY,
}
#[derive(Debug)]
pub enum Recap {
    NORECAP,
    RECAP(String),
}
#[derive(Debug)]
pub enum Output {
    NOOUTPUT,
    OUTPUT(String),
}
#[derive(Debug)]
pub struct Config {
    pub method: Method,
    pub deployment: Deployment,
    pub recap: Recap,
    pub output: Output,
}
impl<'a> Default for Config {
    fn default() -> Self {
        Self {
            method: Method::ASYNC,
            deployment: Deployment::NODEPLOY,
            recap: Recap::NORECAP,
            output: Output::NOOUTPUT,
        }
    }
}
