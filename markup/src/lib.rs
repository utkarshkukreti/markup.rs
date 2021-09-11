pub use markup_proc_macro::{define, new};

mod escape;

pub trait Render {
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result;

    #[doc(hidden)]
    #[inline]
    fn is_none(&self) -> bool {
        false
    }

    #[doc(hidden)]
    #[inline]
    fn is_true(&self) -> bool {
        false
    }

    #[doc(hidden)]
    #[inline]
    fn is_false(&self) -> bool {
        false
    }
}

impl<'a, T: Render + ?Sized> Render for &'a T {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        T::render(self, writer)
    }

    #[doc(hidden)]
    #[inline]
    fn is_none(&self) -> bool {
        T::is_none(self)
    }

    #[doc(hidden)]
    #[inline]
    fn is_true(&self) -> bool {
        T::is_true(self)
    }

    #[doc(hidden)]
    #[inline]
    fn is_false(&self) -> bool {
        T::is_false(self)
    }
}

impl<T: Render + ?Sized> Render for Box<T> {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        T::render(self, writer)
    }

    #[doc(hidden)]
    #[inline]
    fn is_none(&self) -> bool {
        T::is_none(self)
    }

    #[doc(hidden)]
    #[inline]
    fn is_true(&self) -> bool {
        T::is_true(self)
    }

    #[doc(hidden)]
    #[inline]
    fn is_false(&self) -> bool {
        T::is_false(self)
    }
}

impl Render for bool {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(writer, "{}", self)
    }

    #[doc(hidden)]
    #[inline]
    fn is_true(&self) -> bool {
        *self
    }

    #[doc(hidden)]
    #[inline]
    fn is_false(&self) -> bool {
        !self
    }
}

impl<T: Render> Render for Option<T> {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        match self {
            Some(t) => t.render(writer),
            None => Ok(()),
        }
    }

    #[doc(hidden)]
    #[inline]
    fn is_none(&self) -> bool {
        self.is_none()
    }
}

struct Raw<T: std::fmt::Display>(T);

impl<T: std::fmt::Display> Render for Raw<T> {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(writer, "{}", self.0)
    }
}

#[inline]
pub fn raw<T: std::fmt::Display>(t: T) -> impl Render {
    Raw(t)
}

macro_rules! tfor {
    (for $ty:ident in [$($typ:ident),*] $tt:tt) => {
        $( const _: () = { type $ty = $typ; tfor! { @extract $tt } }; )*
    };
    (@extract { $($tt:tt)* }) => { $($tt)* };
}

tfor! {
    for Ty in [char, f32, f64] {
        impl Render for Ty {
            #[inline]
            fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
                write!(writer, "{}", self)
            }
        }
    }
}

tfor! {
    for Ty in [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize] {
        impl Render for Ty {
            #[inline]
            fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
                itoa::fmt(writer, *self)
            }
        }
    }
}

impl Render for str {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        escape::escape(self, writer)
    }
}

impl Render for String {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.as_str().render(writer)
    }
}

macro_rules! tuple_impl {
    ($($ident:ident)+) => {
        impl<$($ident: Render,)+> Render for ($($ident,)+) {
            #[allow(non_snake_case)]
            #[inline]
            fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
                let ($(ref $ident,)+) = *self;
                $($ident.render(writer)?;)+
                Ok(())
            }
        }
    }
}

tuple_impl! { A }
tuple_impl! { A B }
tuple_impl! { A B C }
tuple_impl! { A B C D }
tuple_impl! { A B C D E }
tuple_impl! { A B C D E F }
tuple_impl! { A B C D E F G }
tuple_impl! { A B C D E F G H }
tuple_impl! { A B C D E F G H I }
tuple_impl! { A B C D E F G H I J }

struct Template<F> {
    f: F,
}

pub fn new<'a, F>(f: F) -> impl Render + std::fmt::Display + 'a
where
    F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result + 'a,
{
    Template { f }
}

impl<F> Render for Template<F>
where
    F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result,
{
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        (self.f)(writer)
    }
}

impl<F> std::fmt::Display for Template<F>
where
    F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result,
{
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        Render::render(self, fmt)
    }
}

#[inline]
pub fn doctype() -> impl Render {
    raw("<!DOCTYPE html>")
}
