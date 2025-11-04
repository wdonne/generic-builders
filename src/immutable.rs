/// With this builder you can create a fluent API around an object.
/// # Example
/// ```rust
/// use generic_builders::immutable::Builder;
///
/// #[derive(Debug, PartialEq)]
/// struct Test {
///     field1: i32,
///     field2: i32,
/// }
///
/// assert_eq!(
///     Builder::new(Test {field1: 0, field2: 0})
///         .update(|v| Test {field1: 1, field2: v.field2})
///         .update_if(|v| v.field1 > 0, |v| Test {field1: v.field1, field2: v.field1})
///         .update_if_some(|_| Some(1), |v, n| Test {field1: v.field1 + n, field2: v.field2 + n})
///         .build(),
///     Test {field1: 2, field2: 2})
/// ```
pub struct Builder<T> {
    object: T,
}

impl<T> Builder<T> {
    pub fn new(object: T) -> Self {
        Builder { object }
    }

    pub fn build(self) -> T {
        self.object
    }

    pub fn update<F>(mut self, set: F) -> Builder<T>
    where
        F: Fn(T) -> T,
    {
        self.object = set(self.object);
        self
    }

    pub fn update_if<F, P>(mut self, predicate: P, set: F) -> Builder<T>
    where
        F: Fn(T) -> T,
        P: Fn(&T) -> bool,
    {
        if predicate(&self.object) {
            self.object = set(self.object)
        }

        self
    }

    pub fn update_if_some<F, P, U>(mut self, maybe: P, set: F) -> Builder<T>
    where
        F: Fn(T, &U) -> T,
        P: Fn(&T) -> Option<U>,
    {
        if let Some(val) = maybe(&self.object) {
            self.object = set(self.object, &val);
        }

        self
    }
}
