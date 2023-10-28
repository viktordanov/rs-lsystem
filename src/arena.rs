use std::slice::Iter;
use array_init::array_init;

use crate::token::{Token, TokenId};

#[derive(Debug, Clone)]
pub struct Arena {
    token: [Token; 128],
    rules: [(); 128],
    len: u8,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            token: array_init(|_| Token::default()),
            rules: [(); 128],
            len: 0,
        }
    }

    pub fn len(&self) -> u8 {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get_token(&self, id: &TokenId) -> Option<&Token> {
        self.token.get(id.0 as usize)
    }

    pub fn iter_tokens(&self) -> Iter<'_, Token> {
        // should iterate until length
        self.token[..self.len as usize].iter()
    }

    pub fn is_valid(&self, id: &TokenId) -> bool {
        id.value() < self.len
    }

    pub fn is_valid_slice(&self, slice: &[TokenId]) -> bool {
        slice.iter().all(|id| self.is_valid(id))
    }

    pub fn push_token(&mut self, value: Token) -> TokenId {
        let id = TokenId::new(self.len, value.param() > 0);
        self.token[id.0 as usize] = value;
        self.len += 1;

        id
    }

    pub fn enumerate(&self) -> EnumerableArena {
        EnumerableArena {
            inner: self.iter_tokens().enumerate(),
        }
    }
}

pub struct EnumerableArena<'a> {
    inner: std::iter::Enumerate<Iter<'a, Token>>,
}

impl<'a> Iterator for EnumerableArena<'a> {
    type Item = (TokenId, &'a Token);

    fn next(&mut self) -> Option<Self::Item> {
        let (index, t) = self.inner.next()?;
        Some((TokenId(index as u8), t))
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena_basic() {
        let mut arena = Arena::new();

        let a = arena.push_token("Hello!".into());
        let b = arena.push_token("World".into());

        assert_eq!(a.0, 0);
        assert_eq!(b.0, 1);
        assert_eq!(arena.len(), 2);

        let a_ref = arena.get_token(&a).expect("Failed to get a");

        assert_eq!(*a_ref, "Hello!".into());
        assert_eq!(arena.get_token(&b).unwrap(), &"World".into());
    }

    #[test]
    fn arena_iterator() {
        let mut arena = Arena::new();

        arena.push_token("first".into());
        arena.push_token("second".into());
        arena.push_token("third".into());

        let mut iter = arena.iter_tokens();

        assert_eq!(iter.next(), Some(&"first".into()));
        assert_eq!(iter.next(), Some(&"second".into()));
        assert_eq!(iter.next(), Some(&"third".into()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn arena_enumerate() {
        let mut arena = Arena::new();

        let a = arena.push_token( "first".into());
        let b = arena.push_token( "second".into());
        let c = arena.push_token( "third".into());
        let d = arena.push_token( "fourth".into());

        let mut enumerator = arena.enumerate();

        assert_eq!(enumerator.next(), Some((a, &"first".into())));
        assert_eq!(enumerator.next(), Some((b, &"second".into())));
        assert_eq!(enumerator.next(), Some((c, &"third".into())));
        assert_eq!(enumerator.next(), Some((d, &"fourth".into())));
    }
}
