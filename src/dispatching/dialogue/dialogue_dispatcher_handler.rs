use std::pin::Pin;

use crate::prelude::{DialogueDispatcherHandlerCtx, DialogueStage};
use std::{future::Future, sync::Arc};

/// An asynchronous handler of an update used in [`DialogueDispatcher`].
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
pub trait DialogueDispatcherHandler<Upd, D> {
    #[must_use]
    fn handle(
        self: Arc<Self>,
        ctx: DialogueDispatcherHandlerCtx<Upd, D>,
    ) -> Pin<Box<dyn Future<Output = DialogueStage<D>> + Send + 'static>>
    where
        DialogueDispatcherHandlerCtx<Upd, D>: Send + 'static;
}

impl<Upd, D, F, Fut> DialogueDispatcherHandler<Upd, D> for F
where
    F: Fn(DialogueDispatcherHandlerCtx<Upd, D>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = DialogueStage<D>> + Send + 'static,
{
    fn handle(
        self: Arc<Self>,
        ctx: DialogueDispatcherHandlerCtx<Upd, D>,
    ) -> Pin<Box<dyn Future<Output = Fut::Output> + Send + 'static>>
    where
        DialogueDispatcherHandlerCtx<Upd, D>: Send + 'static,
    {
        Box::pin(async move { self(ctx).await })
    }
}
