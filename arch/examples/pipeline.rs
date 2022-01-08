pub use async_trait::async_trait;
use std::fmt;

pub type BoxedError = Box<dyn std::error::Error>;

/// rerun 超过5次，就视为失败
const MAX_RERUN: usize = 5;

#[must_use]
pub enum PlugResult<Ctx> {
    Continue,
    Rerun,
    Terminate,
    NewPipe(Vec<Box<dyn Plug<Ctx>>>),
    Err(BoxedError),
}

#[async_trait]
pub trait Plug<Ctx>: fmt::Display {
    async fn call(&self, ctx: &mut Ctx) -> PlugResult<Ctx>;
}

#[derive(Default)]
pub struct Pipeline<Ctx> {
    plugs: Vec<Box<dyn Plug<Ctx>>>,
    pos: usize,
    rerun: usize,
    executed: Vec<String>,
}

impl<Ctx> Pipeline<Ctx> {
    /// 创建一个新的pipeline
    pub fn new(plugs: Vec<Box<dyn Plug<Ctx>>>) -> Self {
        Self {
            plugs,
            pos: 0,
            rerun: 0,
            executed: Vec::with_capacity(16),
        }
    }

    /// 执行整个pipeleine，要么执行完毕，要么出错
    pub async fn execute(&mut self, ctx: &mut Ctx) -> Result<(), BoxedError> {
        while self.pos < self.plugs.len() {
            self.add_execution_log();
            let plug = &self.plugs[self.pos];
            match plug.call(ctx).await {
                PlugResult::Continue => {
                    self.pos += 1;
                    self.rerun = 0;
                }
                PlugResult::Rerun => {
                    self.rerun += 1;
                }
                PlugResult::Terminate => break,
                PlugResult::NewPipe(v) => {
                    self.pos = 0;
                    self.rerun = 0;
                    self.plugs = v;
                }
                PlugResult::Err(e) => return Err(e),
            }

            if self.rerun >= MAX_RERUN {
                return Err(anyhow::anyhow!("max_rerun").into());
            }
        }

        todo!()
    }

    pub fn get_execution_log(&self) -> &[String] {
        &self.executed
    }

    fn add_execution_log(&mut self) {
        self.executed.push(self.plugs[self.pos].to_string());
    }
}

// 示例代码
use thiserror::Error;

struct Context;

#[derive(Debug, Error)]
enum MyError {
    #[error("Not found: {0}")]
    NotFound(&'static str),
}

#[derive(Debug)]
struct Normalizer;
struct SecurityChecker;
struct CacheLoader;
struct CacheWriter;
struct DataLoader;

impl fmt::Display for Normalizer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Normalizer")
    }
}

impl fmt::Display for SecurityChecker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SecurityChecked")
    }
}

impl fmt::Display for CacheLoader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CacheLoader")
    }
}

impl fmt::Display for CacheWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CacheWriter")
    }
}

impl fmt::Display for DataLoader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DataLoader")
    }
}

#[async_trait]
impl Plug<Context> for Normalizer {
    async fn call(&self, _ctx: &mut Context) -> PlugResult<Context> {
        PlugResult::Continue
    }
}

#[async_trait]
impl Plug<Context> for SecurityChecker {
    async fn call(&self, _ctx: &mut Context) -> PlugResult<Context> {
        PlugResult::NewPipe(vec![
            Box::new(CacheLoader),
            Box::new(DataLoader),
            Box::new(CacheLoader),
        ])
    }
}

#[async_trait]
impl Plug<Context> for CacheLoader {
    async fn call(&self, _ctx: &mut Context) -> PlugResult<Context> {
        PlugResult::Continue
    }
}

#[async_trait]
impl Plug<Context> for CacheWriter {
    async fn call(&self, _ctx: &mut Context) -> PlugResult<Context> {
        PlugResult::Continue
    }
}

#[async_trait]
impl Plug<Context> for DataLoader {
    async fn call(&self, _ctx: &mut Context) -> PlugResult<Context> {
        PlugResult::Err(Box::new(MyError::NotFound("something")))
    }
}

#[tokio::main]
async fn main() -> Result<(), BoxedError> {
    let mut pipeline = Pipeline::new(vec![Box::new(SecurityChecker), Box::new(Normalizer)]);
    let mut ctx = Context;
    let result = pipeline.execute(&mut ctx).await;
    println!("{:?}", pipeline.get_execution_log());
    println!("{:?}", result);
    Ok(())
}
