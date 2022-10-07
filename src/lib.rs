#![doc = include_str!("../README.md")]

/// Define how `Self` may be overwritten by another types or `Self` itself.
///
/// Note that there are blanket impls for options that you can reuse.
///
/// ```
/// # use overwrite::Overwrite;
///
/// # fn main() {
/// assert_eq!(1.overwrite(Some(2)), 2);
/// assert_eq!(1.overwrite(None), 1);
///
/// // If this behavior is not suitable for you for any reason
/// // you can disable this blanket impl by disabling "overwrite_options"
/// // feature of the crate.
/// assert_eq!(Some(1).overwrite(Some(2)), Some(2));
/// assert_eq!(Some(1).overwrite(None::<i32>), Some(1));
/// assert_eq!(None.overwrite(Some(2)), Some(2));
/// assert_eq!(None::<i32>.overwrite(None::<i32>), None);
/// # }
/// ```
pub trait Overwrite<Rhs = Self> {
    fn overwrite_mut(&mut self, new: Rhs) -> &mut Self;

    fn overwrite(mut self, new: Rhs) -> Self
    where
        Self: Sized,
    {
        self.overwrite_mut(new);
        self
    }
}

impl<T> Overwrite<Option<T>> for T {
    fn overwrite_mut(&mut self, new: Option<T>) -> &mut Self {
        if let Some(val) = new {
            *self = val;
        }

        self
    }
}

#[cfg(feature = "overwrite_options")]
impl<T> Overwrite for Option<T> {
    fn overwrite_mut(&mut self, new: Self) -> &mut Self {
        if let Some(val) = new {
            self.replace(val);
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overwrite() {
        let val1 = None;
        let val2 = Some(6);
        let val3 = Some(7);
        let val4 = None;

        let val = 5
            .overwrite(val1)
            .overwrite(val2)
            .overwrite(val3)
            .overwrite(val4);

        assert_eq!(val, 7);

        let mut opt = None;
        let val1 = None::<u32>;
        let val2 = Some(6);
        let val3 = Some(7);
        let val4 = None::<u32>;

        opt.overwrite_mut(val1)
            .overwrite_mut(val2)
            .overwrite_mut(val3)
            .overwrite_mut(val4);

        assert_eq!(opt, Some(7));
    }
}
