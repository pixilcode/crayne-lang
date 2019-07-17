//! List of lexemes and definition of token metadata
//! 
//! Lexemes are defined as byte constants.
//! 
//! The metadata contained in the token is the offset
//! of the lexeme in the containing string, the line
//! and the column of the lexeme, along with a reference
//! to the lexeme itself.

use crate::parser::internals::Input;
use nom::{
    Compare,
    CompareResult,
    InputTake,
    InputLength,
    InputIter,
    UnspecializedInput
};

/// Macro to help declare tokens
/// 
/// # Example
/// 
/// The following example declares the `EXAMPLE` token
/// 
/// ```
/// token!(EXAMPLE: b"example");
/// ```
macro_rules! token {
    ($name:ident: $value:expr; $documentation:expr) => {
        #[doc=$documentation]
        pub const $name: &'static str = $value;
    };
}

token!(
    COLON: ":";
    "The COLON token.\n\nRepresent the break between identifier and type declaration, eg. `count: Int`"
);
token!(
    COMMA: ",";
    "The COMMA token.\n\nRepresent a separator in a sequence, e.g. `(1, 2, 3)`"
);
token!(
    FN: "fn";
    "The FN token.\n\nRepresent the declaration of a function, e.g. `fn foo() {}`"
);
token!(
    IMPURE: "impure";
    "The IMPURE token.\n\nRepresent an impure function, e.g. `impure fn foo() {}`"
);
token!(
    LEFT_BRACE: "{";
    "The LEFT_BRACE token.\n\nRepresent the beginning of a code block, map, or struct, e.g. `{ ... }`"
);
token!(
    LEFT_PAREN: "(";
    "The LEFT_PAREN token.\n\nRepresent the beginning of a grouping or tuple, e.g. `(a, b)`"
);
token!(
    RETURN_ARROW: "->";
    "The RETURN_ARROW token.\n\nRepresent the return type of a function, e.g. `fn a() -> Int {}`"
);
token!(
    RIGHT_BRACE: "}";
    "The RIGHT_BRACE token.\n\nRepresent the closing of a code block, map, or struct, e.g. `{ ... }`"
);
token!(
    RIGHT_PAREN: ")";
    "The RIGHT_PAREN token.\n\nRepresent the closing of a grouping or tuple, e.g. `(a, b)`"
);

/// A structure pairing data with metadata
pub struct Token<'a, T> {
    /// The value of the token
    value: T,
    
    /// The metadata associated with the token
    meta: Span<'a>
}

impl<'a, T> Token<'a, T> {
    /// Associate a piece of data to metadata
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// use crayne_lang::tokens::{Token, Span};
    /// 
    /// let token = Token::new(1, Span::new(b"1"));
    /// ```
    pub fn new(value: T, meta: Span<'a>) -> Self {
        Token {
            value,
            meta
        }
    }
}

/// Metadata containing details about a token, including
/// offset, location, and lexeme
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Span<'a> {
    /// The position of the lexeme relative to the
    /// beginning of the input of the parser.
    /// 
    /// Offset begins at 0.
    pub offset: usize,
    
    /// The line number of the slice relative to the
    /// beginning of the input of the parser.
    /// 
    /// Line numbering begins at 1.
    pub line: u32,
    
    /// The column number of the slice relative to
    /// the beginning of the line.
    /// 
    /// Column numbering begins at 1.
    pub column: u32,
    
    /// The slice that the metadata describes
    slice: Input<'a>
}

impl<'a> Span<'a> {
    /// Generate token metadata for a slice with default
    /// values for `offset`, `line`, and `column`.
    /// 
    /// * `offset`: 0
    /// * `line`: 1
    /// * `column`: 1
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// use crayne_lang::tokens::Span;
    /// 
    /// let meta = Span::new(b"abc");
    /// 
    /// assert_eq!(0, meta.offset);
    /// assert_eq!(1, meta.line);
    /// assert_eq!(1, meta.column);
    /// assert_eq!(&b"abc"[..], meta.as_slice());
    /// ```
    pub fn new(input: Input<'a>) -> Self {
        Span {
            offset: 0,
            line: 1,
            column: 1,
            slice: input
        }
    }
    
    /// Create token metadata for a slice with supplied
    /// values for `offset`, `line`, and `column`.
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// use crayne_lang::tokens::Span;
    /// 
    /// let meta = Span::new_at(b"abc", 1, 2, 3);
    /// 
    /// assert_eq!(1, meta.offset);
    /// assert_eq!(2, meta.line);
    /// assert_eq!(3, meta.column);
    /// assert_eq!(&b"abc"[..], meta.as_slice());
    /// ```
    pub fn new_at(input: Input<'a>, offset: usize, line: u32, column: u32) -> Self {
        Span {
            offset,
            line,
            column,
            slice: input
        }
    }
    
    /// Create blank metadata
    /// 
    /// This is equivalent to `Span::new(b"")`.
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// use crayne_lang::tokens::Span;
    /// 
    /// let meta = Span::blank();
    /// 
    /// assert_eq!(Span::new(b""), meta);
    /// ```
    pub fn blank() -> Self {
        Self::new("")
    }
    
    /// Get the slice referred to by the metadata
    /// 
    /// # Examples
    /// 
    /// ```ignore
    /// use crayne_lang::tokens::Span;
    /// 
    /// assert_eq!(&b"abc"[..], Span::new(b"abc").as_slice());
    /// ```
    pub fn as_slice(&self) -> Input<'a> {
        self.slice
    }
}

/// Allows comparing of `Span`s to `str`s
/// 
/// Necessary to be used as input for `nom`
impl Compare<&str> for Span<'_> {
    /// Compare the strings contained by each span
    /// (`compare` is already implemented for `&str`)
    #[inline]
    fn compare(&self, t: &str) -> CompareResult {
        self.slice.compare(t)
    }
    
    /// Compare the strings contained by each span
    /// (`compare_no_case` is already implemented for
    ///  `&str`)
    #[inline]
    fn compare_no_case(&self, t: &str) -> CompareResult {
        self.slice.compare_no_case(t)
    }
}

/// Allows taking slices of a `Span`
/// 
/// Necessary to be used as input for `nom`
impl InputTake for Span<'_> {
    
    /// Take a slice from the beginning of the `Span`
    /// to `count`.
    fn take(&self, count: usize) -> Self {
        let slice = &self.slice[..count];
        Span {
            slice,
            ..*self
        }
    }
    
    /// Split the `Span` in half at `count`
    fn take_split(&self, count: usize) -> (Self, Self) {
        let slice_a = &self.slice[..count];
        let slice_b = &self.slice[count..];
        
        let lines = bytecount::count(slice_a.as_bytes(), b'\n') as u32;
        let col_b =
            if lines == 0 {
                self.column + count as u32
            } else {
                match memchr::memrchr(b'\n', slice_a.as_bytes()) {
                    Some(last_newline_position) => {
                        (count - last_newline_position) as u32
                    },
    
                    None => {
                        unreachable!();
                    }
                }
            };
        
        (
            Span {
                offset: self.offset + count,
                line: self.line + lines,
                column: col_b,
                slice: slice_b
            },
            Span {
                slice: slice_a,
                ..*self
            }
        )
    }
}

/// Allows the `nom` parser to determine the
/// length of the span
/// 
/// Necessary to be used as input for `nom`
impl InputLength for Span<'_> {
    /// Return the length of the string
    /// contained by the `Span` (`input_len`
    /// is already defined for `&str`)
    #[inline]
    fn input_len(&self) -> usize {
        self.slice.input_len()
    }
}

/// Generates an iterator for the span
/// 
/// Basically reuses code for `&str` by
/// calling it on the slice
/// 
/// Necessary to be used as input for `nom`
impl<'a> InputIter for Span<'a> {
    type Item = <Input<'a> as InputIter>::Item;
    type Iter = <Input<'a> as InputIter>::Iter;
    type IterElem = <Input<'a> as InputIter>::IterElem;
    
    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.slice.iter_indices()
    }
    
    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
      self.slice.iter_elements()
    }
    
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
      P: Fn(Self::Item) -> bool,
    {
      self.slice.position(predicate)
    }
    
    #[inline]
    fn slice_index(&self, count: usize) -> Option<usize> {
      self.slice.slice_index(count)
    }
}

/// Dummy trait allowing for default implementation
/// of `InputTakeAtPosition`
/// 
/// Necessary to be used as input for `nom`
impl UnspecializedInput for Span<'_> {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn input_take() {
        let span = Span::new("abcde");
        let expected = Span::new("abc");
        
        assert_eq!(expected, span.take(3));
    }
    
    #[test]
    fn input_take_split() {
        let span = Span::new("abcde");
        let expected = (Span::new_at("de", 3, 1, 4), Span::new("abc"));
        
        assert_eq!(expected, span.take_split(3));
    }
}
