use crate::ui::get_i18n;
use anyhow::Result;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

/// SSE line buffering stream
/// Correctly handles split JSON data (a single data: line may be split across byte chunks)
pub struct SseLineStream<S> {
    inner: S,
    buffer: String,
}

impl<S> SseLineStream<S> {
    pub fn new(stream: S) -> Self {
        Self {
            inner: stream,
            buffer: String::new(),
        }
    }
}

impl<S> Stream for SseLineStream<S>
where
    S: Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin,
{
    type Item = Result<String>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            // First check if we have a complete line in the buffer
            if let Some(pos) = self.buffer.find('\n') {
                let line = self.buffer.drain(0..=pos).collect::<String>();
                let trimmed = line.trim_end_matches('\n').to_string();
                if !trimmed.is_empty() || self.buffer.is_empty() {
                    return Poll::Ready(Some(Ok(trimmed)));
                }
            }

            // Get next bytes from the stream
            match Pin::new(&mut self.inner).poll_next(cx) {
                Poll::Ready(Some(Ok(bytes))) => {
                    let text = String::from_utf8_lossy(&bytes);
                    self.buffer.push_str(&text);
                }
                Poll::Ready(Some(Err(e))) => {
                    let i18n = get_i18n();
                    let tmpl = i18n.get("api_stream_error");
                    let msg = tmpl.replace("{}", &e.to_string());
                    return Poll::Ready(Some(Err(anyhow::anyhow!(msg))));
                }
                Poll::Ready(None) => {
                    // Stream ended, send remaining buffer data
                    if !self.buffer.is_empty() {
                        let remaining = std::mem::take(&mut self.buffer);
                        return Poll::Ready(Some(Ok(remaining)));
                    }
                    return Poll::Ready(None);
                }
                Poll::Pending => {
                    return Poll::Pending;
                }
            }
        }
    }
}
