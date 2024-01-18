use std::{
    collections::VecDeque,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use futures::Stream;
use s3s::{
    stream::{ByteStream, RemainingLength},
    StdError,
};

pub(crate) struct VecByteStream {
    queue: VecDeque<Bytes>,
    remaining_bytes: usize,
}

impl VecByteStream {
    pub fn new(v: Vec<Bytes>) -> Self {
        let total = v
            .iter()
            .map(Bytes::len)
            .try_fold(0, usize::checked_add)
            .expect("length overflow");

        Self {
            queue: v.into(),
            remaining_bytes: total,
        }
    }

    #[allow(dead_code)]
    pub fn exact_remaining_length(&self) -> usize {
        self.remaining_bytes
    }
}

impl Stream for VecByteStream {
    type Item = Result<Bytes, StdError>;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = Pin::into_inner(self);
        match this.queue.pop_front() {
            Some(b) => {
                this.remaining_bytes -= b.len();
                Poll::Ready(Some(Ok(b)))
            }
            None => Poll::Ready(None),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let cnt = self.queue.len();
        (cnt, Some(cnt))
    }
}

impl ByteStream for VecByteStream {
    fn remaining_length(&self) -> RemainingLength {
        RemainingLength::new_exact(self.remaining_bytes)
    }
}
