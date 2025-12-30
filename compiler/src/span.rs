use std::ops::Range;

#[macro_export]
macro_rules! span {
    ($t:ident as $s:ident) => {
        pub type $s = crate::span::Spanned<$t>;
        impl $t {
            pub fn spanned(self, span: impl Into<crate::span::Span>) -> crate::span::Spanned<Self> {
                crate::span::Spanned {
                    inner: self,
                    span: span.into(),
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spanned<T> {
    pub inner: T,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

impl From<Span> for Range<usize> {
    fn from(value: Span) -> Self {
        value.start..value.end
    }
}
