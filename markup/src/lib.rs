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
        (*self).render(writer)
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
        *self == true
    }

    #[doc(hidden)]
    #[inline]
    fn is_false(&self) -> bool {
        *self == false
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

pub struct Raw<'a>(&'a str);

impl<'a> Render for Raw<'a> {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        writer.write_str(self.0)
    }
}

#[inline]
pub fn raw(t: &str) -> Raw {
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
