use std::fmt::Write;

pub use markup_proc_macro::{define, new};

mod escape;

pub trait Render {
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result;
}

pub trait RenderAttributeValue: Render {
    #[inline]
    fn is_none(&self) -> bool {
        false
    }

    #[inline]
    fn is_true(&self) -> bool {
        false
    }

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
}

impl<'a, T: RenderAttributeValue + ?Sized> RenderAttributeValue for &'a T {
    #[inline]
    fn is_none(&self) -> bool {
        T::is_none(self)
    }

    #[inline]
    fn is_true(&self) -> bool {
        T::is_true(self)
    }

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
}

impl<T: RenderAttributeValue + ?Sized> RenderAttributeValue for Box<T> {
    #[inline]
    fn is_none(&self) -> bool {
        T::is_none(self)
    }

    #[inline]
    fn is_true(&self) -> bool {
        T::is_true(self)
    }

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
}

impl RenderAttributeValue for bool {
    #[inline]
    fn is_true(&self) -> bool {
        *self
    }

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
}

impl<T: RenderAttributeValue> RenderAttributeValue for Option<T> {
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

impl<T: std::fmt::Display> RenderAttributeValue for Raw<T> {}

#[inline]
pub fn raw(value: impl std::fmt::Display) -> impl RenderAttributeValue {
    Raw(value)
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

        impl RenderAttributeValue for Ty {
        }
    }
}

tfor! {
    for Ty in [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize] {
        impl Render for Ty {
            #[inline]
            fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
                #[cfg(feature = "itoa")] {
                    let mut buffer = itoa::Buffer::new();
                    let str = buffer.format(*self);
                    writer.write_str(str)
                }
                #[cfg(not(feature = "itoa"))] {
                    write!(writer, "{}", self)
                }
            }
        }

        impl RenderAttributeValue for Ty {
        }
    }
}

impl Render for str {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        escape::escape(self, writer)
    }
}

impl RenderAttributeValue for str {}

impl Render for String {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        self.as_str().render(writer)
    }
}

impl RenderAttributeValue for String {}

impl Render for std::fmt::Arguments<'_> {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        escape::Escape(writer).write_fmt(*self)
    }
}

impl RenderAttributeValue for std::fmt::Arguments<'_> {}

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

        impl<$($ident: RenderAttributeValue,)+> RenderAttributeValue for ($($ident,)+) {
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

pub type RenderFn<'a> = dyn Fn(&mut dyn std::fmt::Write) -> std::fmt::Result + 'a;

pub struct DynRender<'a> {
    f: Box<RenderFn<'a>>,
}

pub fn new<'a, F>(f: F) -> DynRender<'a>
where
    F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result + 'a,
{
    DynRender { f: Box::new(f) }
}

impl<'a> Render for DynRender<'a> {
    #[inline]
    fn render(&self, writer: &mut impl std::fmt::Write) -> std::fmt::Result {
        (self.f)(writer)
    }
}

impl<'a> std::fmt::Display for DynRender<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        Render::render(self, fmt)
    }
}

#[inline]
pub fn doctype() -> impl Render {
    raw("<!DOCTYPE html>")
}
