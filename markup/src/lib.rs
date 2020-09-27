use std::fmt::Display;

pub use markup_proc_macro::{define, render};

mod escape;

pub trait Render {
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result;

    #[inline]
    fn is_none(&self) -> bool {
        false
    }
}

impl<'a, T: Render + ?Sized> Render for &'a T {
    #[inline]
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        (*self).render(w)
    }
}

impl<T: Render> Render for Option<T> {
    #[inline]
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        match self {
            Some(t) => t.render(w),
            None => Ok(()),
        }
    }

    #[inline]
    fn is_none(&self) -> bool {
        self.is_none()
    }
}

impl Render for str {
    #[inline]
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        escape::escape(self, w)
    }
}

impl Render for String {
    #[inline]
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.as_str().render(w)
    }
}

struct Raw<T: Display>(pub T);

impl<T: Display> Render for Raw<T> {
    #[inline]
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(w, "{}", self.0)
    }
}

#[inline]
pub fn raw<T: Display>(t: T) -> impl Render {
    Raw(t)
}

macro_rules! display {
    ($($ty:ty)*) => {
        $(
            impl Render for $ty {
                #[inline]
                fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
                    write!(w, "{}", self)
                }
            }
        )*
    };
}

display! {
    bool
    char
    f32 f64
}

macro_rules! itoa {
    ($($ty:ty)*) => {
        $(
            impl Render for $ty {
                #[inline]
                fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
                    itoa::fmt(w, *self)
                }
            }
        )*
    };
}

itoa! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
}

#[inline]
pub fn doctype() -> impl Render {
    raw("<!DOCTYPE html>")
}

pub struct Template<F>(pub F)
where
    F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result;

impl<F> std::fmt::Display for Template<F>
where
    F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result,
{
    #[inline]
    fn fmt(&self, __fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self.0)(__fmt)
    }
}

impl<F> Render for Template<F>
where
    F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result,
{
    #[inline]
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(w, "{}", self)
    }
}
