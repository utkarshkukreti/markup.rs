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

macro_rules! impl_render_with {
    ($([$($ty:ty)+] => |$self_:ident, $writer:ident| $expr:expr,)+) => {
        $(
            $(
                impl Render for $ty {
                    #[inline]
                    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
                        let ($self_, $writer) = (self, writer);
                        $expr
                    }
                }
            )+
        )+
    };
}

impl_render_with! {
    [char f32 f64] => |self_, writer| write!(writer, "{}", self_),
    [u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize] => |self_, writer| itoa::fmt(writer, *self_),
    [str] => |self_, writer| escape::escape(self_, writer),
    [String] => |self_, writer| self_.as_str().render(writer),
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
