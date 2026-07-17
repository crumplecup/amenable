//! Calculation interface: pointing at code that performs a check, rather
//! than duplicating its logic.

/// A return type that carries a proof-of-work token alongside whatever data
/// it produces, whether that's a bare token, a tuple with data, or a
/// `Result`.
pub trait CarriesToken {
    /// Proof-of-work token type, conventionally zero-sized and mintable
    /// only by a successful calculation.
    type Token;

    /// Borrow the token carried by this output.
    fn token(&self) -> &Self::Token;
}

/// A calculation is any callable whose output carries a token proving it
/// ran. Implemented for `FnMut` of any arity via macro, so a calculation may
/// take zero, one, or many arguments.
pub trait Calculation<Args> {
    /// Computed output, which must carry a proof-of-work token.
    type Output: CarriesToken;

    /// Perform the calculation.
    fn calculate(&mut self, args: Args) -> Self::Output;
}

macro_rules! impl_calculation {
    ($($arg:ident : $var:ident),*) => {
        impl<F, O, $($arg),*> Calculation<($($arg,)*)> for F
        where
            F: FnMut($($arg),*) -> O,
            O: CarriesToken,
        {
            type Output = O;

            fn calculate(&mut self, args: ($($arg,)*)) -> Self::Output {
                let ($($var,)*) = args;
                self($($var),*)
            }
        }
    };
}

impl_calculation!();
impl_calculation!(A1: a1);
impl_calculation!(A1: a1, A2: a2);
impl_calculation!(A1: a1, A2: a2, A3: a3);
impl_calculation!(A1: a1, A2: a2, A3: a3, A4: a4);
