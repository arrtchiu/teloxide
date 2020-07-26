use crate::{
    dispatching::{dialogue::DialogueStage, UpdateWithCx},
    requests::ResponseResult,
    types::Message,
};
use futures::future::BoxFuture;

/// Represents a transition function of a dialogue FSM.
pub trait Transition: Sized {
    /// Turns itself into another state, depending on the input message.
    fn react(self, cx: TransitionIn)
        -> BoxFuture<'static, TransitionOut<Self>>;
}

/// Like [`Transition`], but from `StateN` -> `Dialogue`.
///
/// [`Transition`]: crate::dispatching::dialogue::Transition
pub trait SubTransition<Dialogue>
where
    Dialogue: Transition,
{
    /// Turns itself into another state, depending on the input message.
    fn react(
        self,
        cx: TransitionIn,
    ) -> BoxFuture<'static, TransitionOut<Dialogue>>;
}

/// A type returned from a FSM subtransition function.
///
/// Now it is used only inside `#[teloxide(transition)]` for type inference.
pub trait SubTransitionOutputType {
    type Output;
}

impl<D> SubTransitionOutputType for TransitionOut<D> {
    type Output = D;
}

/// An input passed into a FSM (sub)transition function.
pub type TransitionIn = UpdateWithCx<Message>;

/// A type returned from a FSM (sub)transition function.
pub type TransitionOut<D> = ResponseResult<DialogueStage<D>>;
