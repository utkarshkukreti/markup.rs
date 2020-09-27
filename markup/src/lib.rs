use std::fmt::Display;

pub use markup_proc_macro::{define, render};

mod escape;

pub trait Render {
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result;

    fn is_none(&self) -> bool {
        false
    }
}

impl<'a, T: Render + ?Sized> Render for &'a T {
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        (*self).render(w)
    }
}

impl<T: Render> Render for Option<T> {
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        match self {
            Some(t) => t.render(w),
            None => Ok(()),
        }
    }

    fn is_none(&self) -> bool {
        self.is_none()
    }
}

impl Render for str {
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        escape::escape(self, w)
    }
}

impl Render for String {
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.as_str().render(w)
    }
}

struct Raw<T: Display>(pub T);

impl<T: Display> Render for Raw<T> {
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(w, "{}", self.0)
    }
}

pub fn raw<T: Display>(t: T) -> impl Render {
    Raw(t)
}

macro_rules! raw_display {
    ($($ty:ty)*) => {
        $(
            impl Render for $ty {
                fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
                    write!(w, "{}", self)
                }
            }
        )*
    };
}

raw_display! {
    bool
    char
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    f32 f64
}

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
    fn fmt(&self, __fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        (self.0)(__fmt)
    }
}

impl<F> Render for Template<F>
where
    F: Fn(&mut std::fmt::Formatter) -> std::fmt::Result,
{
    fn render(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        write!(w, "{}", self)
    }
}
