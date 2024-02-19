pub trait GenZOption {
    type Inner;

    /////////////////////////////////////////////////////////////////////////
    // Querying the contained values
    /////////////////////////////////////////////////////////////////////////

    /// Returns `true` if the option is a [`Some`] value.
    ///
    /// See [`Option::is_ok`](std::option::Option::is_some) documentation for more details.
    fn no_cap(&self) -> bool;

    /// Returns `true` if the option is a [`Some`] and the value inside of it matches a predicate.
    ///
    /// See [`Option::is_ok_and`](std::option::Option::is_some_and) documentation for more details.
    fn no_cap_and(self, f: impl FnOnce(Self::Inner) -> bool) -> bool;

    /// Returns `true` if the option is a [`None`] value.
    ///
    /// See [`Option::is_err`](std::option::Option::is_none) documentation for more details.
    fn cap(&self) -> bool;

    /////////////////////////////////////////////////////////////////////////
    // Transforming contained values
    /////////////////////////////////////////////////////////////////////////

    /// Maps an `Option<T>` to `Option<U>` by applying a function to a contained value (if `Some`) or returns `None` (if `None`).
    ///
    /// This function can be used to compose the results of two functions.
    ///
    /// See [`Option::map`](std::option::Option::map) documentation for more details.
    fn glow_up<U, F: FnOnce(Self::Inner) -> U>(self, op: F) -> Option<U>;

    /// Returns the provided default result (if none),
    /// or applies a function to the contained value (if any).
    ///
    /// Arguments passed to `glow_up_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use `on_god_or_else`,
    /// which is lazily evaluated.
    ///
    /// [`glow_up_or_else`]: GenZOption::glow_up_or_else
    ///
    /// See [`Option::map_or`](std::option::Option::map_or) documentation for more details.
    fn glow_up_or<U, F: FnOnce(Self::Inner) -> U>(self, default: U, f: F) -> U;

    /// Computes a default function result (if none), or
    /// applies a different function to the contained value (if any).
    ///
    /// See [`Option::map_or_else`](std::option::Option::map_or_else) documentation for more details.
    fn glow_up_or_else<U, D: FnOnce() -> U, F: FnOnce(Self::Inner) -> U>(
        self,
        default: D,
        f: F,
    ) -> U;

    /// Transforms the `Option<T>` into a [`Result<T, E>`], mapping [`Some(v)`] to
    /// [`Ok(v)`] and [`None`] to [`Err(err)`].
    ///
    /// Arguments passed to `bet_or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`bet_or_else`], which is
    /// lazily evaluated.
    ///
    /// [`Ok(v)`]: Ok
    /// [`Err(err)`]: Err
    /// [`Some(v)`]: Some
    /// [`bet_or_else`]: GenZOption::bet_or_else
    ///
    /// See [`Option::ok_or`](std::option::Option::ok_or) documentation for more details.
    fn bet_or<E>(self, err: E) -> Result<Self::Inner, E>;

    /// Transforms the `Option<T>` into a [`Result<T, E>`], mapping [`Some(v)`] to
    /// [`Ok(v)`] and [`None`] to [`Err(err())`].
    ///
    /// [`Ok(v)`]: Ok
    /// [`Err(err())`]: Err
    /// [`Some(v)`]: Some
    ///
    /// See [`Option::ok_or_else`](std::option::Option::ok_or_else) documentation for more details.
    fn bet_or_else<E, F>(self, err: F) -> Result<Self::Inner, E>
    where
        F: FnOnce() -> E;

    /////////////////////////////////////////////////////////////////////////
    // Getting to contained values
    /////////////////////////////////////////////////////////////////////////

    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`None`] with a custom panic message provided by
    /// `msg`.
    ///
    /// See [`Option::expect`](std::option::Option::expect) documentation for more details.
    fn based(self, msg: &str) -> Self::Inner;

    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`None`]
    /// case explicitly, or call [`on_god_or`], [`on_god_or_else`], or
    /// [`on_god_or_basic`].
    ///
    /// [`on_god_or`]: GenZOption::on_god_or
    /// [`on_god_or_else`]: GenZOption::on_god_or_else
    /// [`on_god_or_basic`]: GenZOption::on_god_or_basic
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`None`].
    ///
    /// See [`Option::unwrap`](std::option::Option::unwrap) documentation for more details.
    fn on_god(self) -> Self::Inner;

    /// Returns the contained [`Some`] value or a provided default.
    ///
    /// Arguments passed to `on_god_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`on_god_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`on_god_or_else`]: GenZOption::on_god_or_else
    ///
    /// See [`Option::unwrap_or`](std::option::Option::unwrap_or) documentation for more details.
    fn on_god_or(self, default: Self::Inner) -> Self::Inner;

    /// Returns the contained [`Some`] value or computes it from a closure.
    ///
    /// See [`Option::unwrap_or_else`](std::option::Option::unwrap_or_else) documentation for more details.
    fn on_god_or_else<F: FnOnce() -> Self::Inner>(self, op: F) -> Self::Inner;

    /// Returns the contained [`Some`] value or a default
    ///
    /// Consumes the `self` argument then, if [`Some`], returns the contained
    /// value, otherwise if [`None`], returns the default value for that
    /// type.
    ///
    /// See [`Option::unwrap_or_default`](std::option::Option::unwrap_or_default) documentation for more details.
    fn on_god_or_basic(self) -> Self::Inner
    where
        Self::Inner: Default;
}

impl<T> GenZOption for Option<T> {
    type Inner = T;

    fn no_cap(&self) -> bool {
        self.is_some()
    }

    fn no_cap_and(self, f: impl FnOnce(Self::Inner) -> bool) -> bool {
        self.is_some_and(f)
    }

    fn cap(&self) -> bool {
        self.is_none()
    }

    fn glow_up<U, F: FnOnce(Self::Inner) -> U>(self, f: F) -> Option<U> {
        self.map(f)
    }

    fn glow_up_or<U, F: FnOnce(Self::Inner) -> U>(self, default: U, f: F) -> U {
        self.map_or(default, f)
    }

    fn glow_up_or_else<U, D: FnOnce() -> U, F: FnOnce(Self::Inner) -> U>(
        self,
        default: D,
        f: F,
    ) -> U {
        self.map_or_else(default, f)
    }

    fn bet_or<E>(self, err: E) -> Result<Self::Inner, E> {
        self.ok_or(err)
    }

    fn bet_or_else<E, F>(self, err: F) -> Result<Self::Inner, E>
    where
        F: FnOnce() -> E,
    {
        self.ok_or_else(err)
    }

    fn based(self, msg: &str) -> Self::Inner {
        self.expect(msg)
    }

    fn on_god(self) -> Self::Inner {
        self.unwrap()
    }

    fn on_god_or(self, default: Self::Inner) -> Self::Inner {
        self.unwrap_or(default)
    }

    fn on_god_or_else<F: FnOnce() -> Self::Inner>(self, f: F) -> Self::Inner {
        self.unwrap_or_else(f)
    }

    fn on_god_or_basic(self) -> Self::Inner
    where
        Self::Inner: Default,
    {
        self.unwrap_or_default()
    }
}

pub trait GenZNestedOption: GenZOption {
    type InnerInner;
    /// Converts from `Option<Option<T>>` to `Option<T>`.
    ///
    /// Flattening only removes one level of nesting at a time:
    ///
    /// See [`Option::flatten`](std::option::Option::flatten) documentation for more details.
    fn on_a_stack(self) -> Option<Self::InnerInner>;
}

impl<T> GenZNestedOption for Option<Option<T>> {
    type InnerInner = T;
    fn on_a_stack(self) -> Option<Self::InnerInner> {
        self.flatten()
    }
}
