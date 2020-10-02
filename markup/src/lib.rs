pub use markup_proc_macro::{define, new};

mod escape;

pub trait Render {
    fn write_to(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result;

    #[inline]
    fn is_none(&self) -> bool {
        false
    }
}

impl<'a, T: Render + ?Sized> Render for &'a T {
    #[inline]
    fn write_to(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        (*self).write_to(w)
    }
}

impl<T: Render> Render for Option<T> {
    #[inline]
    fn write_to(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        match self {
            Some(t) => t.write_to(w),
            None => Ok(()),
        }
    }

    #[inline]
    fn is_none(&self) -> bool {
        self.is_none()
    }
}

pub struct Raw<'a>(&'a str);

impl<'a> Render for Raw<'a> {
    #[inline]
    fn write_to(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        w.write_str(self.0)
    }
}

#[inline]
pub fn raw(t: &str) -> Raw {
    Raw(t)
}

macro_rules! impl_render_with {
    ($([$($ty:ty)+] => |$self_:ident, $w:ident| $expr:expr,)+) => {
        $(
            $(
                impl Render for $ty {
                    #[inline]
                    fn write_to(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
                        let ($self_, $w) = (self, w);
                        $expr
                    }
                }
            )+
        )+
    };
}

impl_render_with! {
    [bool char f32 f64] => |self_, w| write!(w, "{}", self_),
    [u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize] => |self_, w| itoa::fmt(w, *self_),
    [str] => |self_, w| escape::escape(self_, w),
    [String] => |self_, w| self_.as_str().write_to(w),
}

struct Dynamic<F> {
    f: F,
}

pub fn new<'a, F>(f: F) -> impl Render + std::fmt::Display + 'a
where
    F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result + 'a,
{
    Dynamic { f }
}

impl<F> Render for Dynamic<F>
where
    F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result,
{
    #[inline]
    fn write_to(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        (self.f)(w)
    }
}

impl<F> std::fmt::Display for Dynamic<F>
where
    F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result,
{
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        Render::write_to(self, fmt)
    }
}

#[inline]
pub fn doctype() -> impl Render {
    raw("<!DOCTYPE html>")
}
