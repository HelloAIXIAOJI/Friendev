use std::io;
use std::sync::OnceLock;

type ReviewHandler = dyn Fn(&ReviewRequest) -> io::Result<()> + Send + Sync + 'static;

static REVIEW_HANDLER: OnceLock<Box<ReviewHandler>> = OnceLock::new();

pub struct ReviewRequest<'a> {
    pub action: &'a str,
    pub subject: &'a str,
    pub preview: Option<&'a str>,
}

pub fn set_review_handler<F>(handler: F)
where
    F: Fn(&ReviewRequest) -> io::Result<()> + Send + Sync + 'static,
{
    let _ = REVIEW_HANDLER.set(Box::new(handler));
}
