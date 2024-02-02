use std::future::Future;

// https://github.com/rust-lang/rust/issues/98712
// https://github.com/cameronelliott/await-coverage-workaround

#[allow(dead_code)]
#[derive(Debug, Default)]
struct Yield {
    init: bool,
}

impl std::future::Future for Yield {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if !self.init {
            self.init = true;
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
        } else {
            return std::task::Poll::Ready(());
        }
    }
}

pub(crate) trait FixAwaitCoverage {
    async fn fix_cov(self) -> <Self as Future>::Output
    where
        Self: Sized,
        Self: Future,
    {
        // this will NOT show as covered
        // but for my usage I just keep it outside of my coverage checked code
        let r = self.await;
        Yield::default().await;
        r
    }
}

impl<F, T> FixAwaitCoverage for F where F: Future<Output = T> {}
