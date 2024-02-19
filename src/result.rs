use core::fmt;

pub trait GenZResult {
    type Inner;
    type ErrorType;

    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the result is [`Ok`].
    ///
    /// See [`Result::is_ok`](std::result::Result::is_ok) documentation for more details.
    fn no_cap(&self) -> bool;

    /// Returns `true` if the result is [`Ok`] and the value inside of it matches a predicate.
    ///
    /// See [`Result::is_ok_and`](std::result::Result::is_ok_and) documentation for more details.
    fn no_cap_and(self, f: impl FnOnce(Self::Inner) -> bool) -> bool;

    /// Returns `true` if the result is [`Err`].
    ///
    /// See [`Result::is_err`](std::result::Result::is_err) documentation for more details.
    fn cap(&self) -> bool;

    /// Returns `true` if the result is [`Err`] and the value inside of it matches a predicate.
    ///
    /// See [`Result::is_err_and`](std::result::Result::is_err_and) documentation for more details.
    fn cap_and(self, f: impl FnOnce(Self::ErrorType) -> bool) -> bool;

    /////////////////////////////////////////////////////////////////////////
    // Adapter for each variant
    /////////////////////////////////////////////////////////////////////////

    /// Converts from `Result<Self::Inner, Self::ErrorType>` to [`Option<Self::Inner>`].
    ///
    /// Converts `self` into an [`Option<Self::Inner>`], consuming `self`,
    /// and discarding the error, if any.
    ///
    /// See [`Result::ok`](std::result::Result::ok) documentation for more details.
    fn bet(self) -> Option<Self::Inner>;

    /////////////////////////////////////////////////////////////////////////
    // Transforming contained values
    /////////////////////////////////////////////////////////////////////////

    /// Maps a `Result<Self::Inner, Self::ErrorType>` to `Result<U, Self::ErrorType>` by applying a function to a
    /// contained [`Ok`] value, leaving an [`Err`] value untouched.
    ///
    /// This function can be used to compose the results of two functions.
    ///
    /// See [`Result::map`](std::result::Result::map) documentation for more details.
    fn glow_up<U, F: FnOnce(Self::Inner) -> U>(self, op: F) -> Result<U, Self::ErrorType>;

    /// Returns the provided default (if [`Err`]), or
    /// applies a function to the contained value (if [`Ok`]).
    ///
    /// Arguments passed to `glow_up_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`glow_up_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`glow_up_or_else`]: GenZResult::glow_up_or_else
    ///
    /// See [`Result::map_or`](std::result::Result::map_or) documentation for more details.
    fn glow_up_or<U, F: FnOnce(Self::Inner) -> U>(self, default: U, f: F) -> U;

    /// Maps a `Result<Self::Inner, Self::ErrorType>` to `U` by applying fallback function `default` to
    /// a contained [`Err`] value, or function `f` to a contained [`Ok`] value.
    ///
    /// This function can be used to unpack a successful result
    /// while handling an error.
    ///
    ///
    /// See [`Result::map_or_else`](std::result::Result::map_or_else) documentation for more details.
    fn glow_up_or_else<U, D: FnOnce(Self::ErrorType) -> U, F: FnOnce(Self::Inner) -> U>(
        self,
        default: D,
        f: F,
    ) -> U;

    /// Maps a `Result<Self::Inner, Self::ErrorType>` to `Result<Self::Inner, F>` by applying a function to a
    /// contained [`Err`] value, leaving an [`Ok`] value untouched.
    ///
    /// This function can be used to pass through a successful result while handling
    /// an error.
    ///
    ///
    /// See [`Result::map_err`](std::result::Result::map_err) documentation for more details.
    fn vibe_check<F, O: FnOnce(Self::ErrorType) -> F>(self, op: O) -> Result<Self::Inner, F>;

    /////////////////////////////////////////////////////////////////////////
    // Extract a value
    /////////////////////////////////////////////////////////////////////////

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Err`]
    /// case explicitly, or call [`on_god_or`], [`on_god_or_else`], or
    /// [`on_god_or_default`].
    ///
    /// [`on_god_or`]: GenZResult::on_god_or
    /// [`on_god_or_else`]: GenZResult::on_god_or_else
    /// [`on_god_or_default`]: GenZResult::on_god_or_default
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a panic message including the
    /// passed message, and the content of the [`Err`].
    ///
    ///
    /// See [`Result::expect`](std::result::Result::expect) documentation for more details.
    fn based(self, msg: &str) -> Self::Inner
    where
        Self::ErrorType: fmt::Debug;

    /// Returns the contained [`Ok`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Err`]
    /// case explicitly, or call [`on_god_or`], [`on_god_or_else`], or
    /// [`on_god_or_default`].
    ///
    /// [`on_god_or`]: GenZResult::on_god_or
    /// [`on_god_or_else`]: GenZResult::on_god_or_else
    /// [`on_god_or_default`]: GenZResult::on_god_or_default
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Err`], with a panic message provided by the
    /// [`Err`]'s value.
    ///
    ///
    /// See [`Result::unwrap`](std::result::Result::unwrap) documentation for more details.
    fn on_god(self) -> Self::Inner
    where
        Self::ErrorType: fmt::Debug;

    /// Returns the contained [`Ok`] value or a default
    ///
    /// Consumes the `self` argument then, if [`Ok`], returns the contained
    /// value, otherwise if [`Err`], returns the default value for that
    /// type.
    ///
    /// See [`Result::unwrap_or_default`](std::result::Result::unwrap_or_default) documentation for more details.
    fn on_god_or_basic(self) -> Self::Inner
    where
        Self::Inner: Default;

    /// Returns the contained [`Err`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], with a panic message including the
    /// passed message, and the content of the [`Ok`].
    ///
    ///
    /// See [`Result::expect_err`](std::result::Result::expect_err) documentation for more details.
    fn out_of_pocket(self, msg: &str) -> Self::ErrorType
    where
        Self::Inner: fmt::Debug;

    /// Returns the contained [`Err`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Ok`], with a custom panic message provided
    /// by the [`Ok`]'s value.
    ///
    /// See [`Result::unwrap_err`](std::result::Result::unwrap_err) documentation for more details.
    fn big_yikes(self) -> Self::ErrorType
    where
        Self::Inner: fmt::Debug;

    /// Returns the contained [`Ok`] value or a provided default.
    ///
    /// Arguments passed to `on_god_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`on_god_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`on_god_or_else`]: GenZResult::on_god_or_else
    ///
    /// See [`Result::unwrap_or`](std::result::Result::unwrap_or) documentation for more details.
    fn on_god_or(self, default: Self::Inner) -> Self::Inner;

    /// Returns the contained [`Ok`] value or computes it from a closure.
    ///
    /// See [`Result::unwrap_or_else`](std::result::Result::unwrap_or_else) documentation for more details.
    fn on_god_or_else<F: FnOnce(Self::ErrorType) -> Self::Inner>(self, op: F) -> Self::Inner;
}

impl<T, E> GenZResult for Result<T, E> {
    type Inner = T;
    type ErrorType = E;

    fn no_cap(&self) -> bool {
        self.is_ok()
    }

    fn no_cap_and(self, f: impl FnOnce(Self::Inner) -> bool) -> bool {
        self.is_ok_and(f)
    }

    fn cap(&self) -> bool {
        self.is_err()
    }

    fn cap_and(self, f: impl FnOnce(Self::ErrorType) -> bool) -> bool {
        self.is_err_and(f)
    }

    fn bet(self) -> Option<Self::Inner> {
        self.ok()
    }

    fn glow_up<U, F: FnOnce(Self::Inner) -> U>(self, op: F) -> Result<U, Self::ErrorType> {
        self.map(op)
    }

    fn glow_up_or<U, F: FnOnce(Self::Inner) -> U>(self, default: U, f: F) -> U {
        self.map_or(default, f)
    }

    fn glow_up_or_else<U, D: FnOnce(Self::ErrorType) -> U, F: FnOnce(Self::Inner) -> U>(
        self,
        default: D,
        f: F,
    ) -> U {
        self.map_or_else(default, f)
    }

    fn vibe_check<F, O: FnOnce(Self::ErrorType) -> F>(self, op: O) -> Result<Self::Inner, F> {
        self.map_err(op)
    }

    fn based(self, msg: &str) -> Self::Inner
    where
        Self::ErrorType: fmt::Debug,
    {
        self.expect(msg)
    }

    fn on_god(self) -> Self::Inner
    where
        Self::ErrorType: fmt::Debug,
    {
        self.unwrap()
    }

    fn on_god_or_basic(self) -> Self::Inner
    where
        Self::Inner: Default,
    {
        self.unwrap_or_default()
    }

    fn out_of_pocket(self, msg: &str) -> Self::ErrorType
    where
        Self::Inner: fmt::Debug,
    {
        self.expect_err(msg)
    }

    fn big_yikes(self) -> Self::ErrorType
    where
        Self::Inner: fmt::Debug,
    {
        self.unwrap_err()
    }

    fn on_god_or(self, default: Self::Inner) -> Self::Inner {
        self.unwrap_or(default)
    }

    fn on_god_or_else<F: FnOnce(Self::ErrorType) -> Self::Inner>(self, op: F) -> Self::Inner {
        self.unwrap_or_else(op)
    }
}

// Disabled because [`Issue #70142`](https://github.com/rust-lang/rust/issues/70142)
// pub trait GenZNestedResult: GenZResult {
//     type InnerInner;
//     /// Converts from `Result<Result<T, E>, E>` to `Result<T, E>`
//     ///
//     /// Flattening only removes one level of nesting at a time:
//     ///
//     /// See [`Result::flatten`](std::result::Result::flatten) documentation for more details.
//     fn on_a_stack(self) -> Result<Self::InnerInner, Self::ErrorType>;
// }
//
// impl<T, E> GenZNestedResult for Result<Result<T, E>, E> {
//     type InnerInner = T;
//     fn on_a_stack(self) -> Result<Self::InnerInner, Self::ErrorType> {
//         self.flatten()
//     }
// }
