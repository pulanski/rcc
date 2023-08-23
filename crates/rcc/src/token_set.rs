use crate::{cst::SyntaxKind, lexer::TokenKind};

/// A _memory-efficient_ **bit-set** for _managing_ and _querying_ a **collection** of [`SyntaxKind`]s /
/// [`TokenKind`]s.
///
/// The [`TokenSet`] struct is a **space-efficient** data structure for working with collections of
/// [`SyntaxKind`]s. It is designed for performance and provides many utility APIs for _quickly
/// checking_ **membership**, **union**, **intersection**, and **other set operations**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TokenSet(u128);

impl From<Vec<TokenKind>> for TokenSet {
    /// Creates a new [`TokenSet`] containing the given [`TokenKind`]s.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::TokenKind;
    ///
    /// let set = TokenSet::from(&[TokenKind::Identifier, TokenKind::Keyword]);
    /// assert!(set.contains(TokenKind::Identifier));
    /// assert!(set.contains(TokenKind::Keyword));
    /// ```
    fn from(kinds: Vec<TokenKind>) -> Self {
        let mut res = 0u128;
        let mut i = 0;
        while i < kinds.len() {
            res |= mask(kinds[i].to_syntax());
            i += 1;
        }
        TokenSet(res)
    }
}

impl TokenSet {
    /// An empty `TokenSet`.
    pub const EMPTY: TokenSet = TokenSet(0);

    /// Creates a new [`TokenSet`] containing the given [`SyntaxKind`]s.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    /// ```
    pub const fn new(kinds: &[SyntaxKind]) -> TokenSet {
        let mut res = 0u128;
        let mut i = 0;
        while i < kinds.len() {
            res |= mask(kinds[i]);
            i += 1;
        }
        TokenSet(res)
    }

    /// Returns a new [`TokenSet`] containing the union of `self` and `other`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let set1 = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// let set2 = TokenSet::new(&[SyntaxKind::Punctuation, SyntaxKind::Keyword]);
    ///
    /// let union_set = set1.union(set2);
    /// assert!(union_set.contains(SyntaxKind::Identifier));
    /// assert!(union_set.contains(SyntaxKind::Punctuation));
    /// assert!(union_set.contains(SyntaxKind::Keyword));
    /// ```
    pub const fn union(self, other: TokenSet) -> TokenSet {
        TokenSet(self.0 | other.0)
    }

    /// Returns `true` if the [`TokenSet`] contains the specified [`SyntaxKind`].
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let set = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// assert!(set.contains(SyntaxKind::Identifier));
    /// assert!(!set.contains(SyntaxKind::Punctuation));
    /// ```
    pub const fn contains(&self, kind: TokenKind) -> bool {
        self.0 & mask(kind.to_syntax()) != 0
    }

    pub const fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let mut set = TokenSet::new(&[SyntaxKind::Identifier]);
    /// set.merge(SyntaxKind::Keyword);
    ///
    /// assert!(set.contains(SyntaxKind::Identifier));
    /// assert!(set.contains(SyntaxKind::Keyword));
    /// ```
    pub fn merge(&mut self, kind: TokenKind) {
        self.0 |= mask(kind.to_syntax());
    }

    /// Returns `true` if the `TokenSet` is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    ///
    /// let empty_set = TokenSet::EMPTY;
    /// assert!(empty_set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    //   /// Returns an iterator over the [`SyntaxKind`]s in the `TokenSet`.
    //   ///
    //   /// # Example
    //   ///
    //   /// ```
    //   /// use crate::TokenSet;
    //   /// use crate::SyntaxKind;
    //   ///
    //   /// let set = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    //   /// let mut iter = set.kinds();
    //   ///
    //   /// assert_eq!(iter.next(), Some(SyntaxKind::Identifier));
    //   /// assert_eq!(iter.next(), Some(SyntaxKind::Keyword));
    //   /// assert_eq!(iter.next(), None);
    //   /// ```
    //   pub fn kinds(self) -> impl Iterator<Item = TokenKind> {
    //     // (0..=SyntaxKind::__LAST.into())
    //     //   .map(SyntaxKind::from)
    //     //   .filter(move |it| self.contains(*it))
    //     TokenKind::all().filter(move |it| self.contains(*it))
    //   }

    //   /// An **alias** for the [`kinds`] method, which returns an **iterator** over the [`SyntaxKind`]s
    //   /// in the [`TokenSet`].
    //   pub fn iter(self) -> impl Iterator<Item = TokenKind> {
    //     self.kinds()
    //   }

    /// Returns `true` if `self` is a subset of `other`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let set1 = TokenSet::new(&[SyntaxKind::Identifier]);
    /// let set2 = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    ///
    /// assert!(set1.is_subset(set2));
    /// assert!(!set2.is_subset(set1));
    /// ```
    pub fn is_subset(&self, other: TokenSet) -> bool {
        (self.0 & other.0) == self.0
    }

    /// Returns `true` if `self` and `other` have no common elements.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let set1 = TokenSet::new(&[SyntaxKind::Identifier]);
    /// let set2 = TokenSet::new(&[SyntaxKind::Keyword]);
    ///
    /// assert!(set1.is_disjoint(set2));
    /// ```
    pub fn is_disjoint(&self, other: TokenSet) -> bool {
        (self.0 & other.0) == 0
    }

    /// Returns a new `TokenSet` containing the
    /// intersection of `self` and `other`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let set1 = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// let set2 = TokenSet::new(&[SyntaxKind::Keyword, SyntaxKind::Punctuation]);
    ///
    /// let intersection_set = set1.intersection(set2);
    /// assert!(!intersection_set.contains(SyntaxKind::Identifier));
    /// assert!(intersection_set.contains(SyntaxKind::Keyword));
    /// assert!(!intersection_set.contains(SyntaxKind::Punctuation));
    /// ```
    pub fn intersection(&self, other: TokenSet) -> TokenSet {
        TokenSet(self.0 & other.0)
    }

    /// Removes the elements of `other` from `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let mut set1 = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// let set2 = TokenSet::new(&[SyntaxKind::Keyword]);
    ///
    /// set1.remove(set2);
    /// assert!(set1.contains(SyntaxKind::Identifier));
    /// assert!(!set1.contains(SyntaxKind::Keyword));
    /// ```
    pub fn remove(&mut self, other: TokenSet) {
        self.0 &= !other.0;
    }

    /// Inserts the elements of `other` into `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let mut set1 = TokenSet::new(&[SyntaxKind::Identifier]);
    /// let set2 = TokenSet::new(&[SyntaxKind::Keyword]);
    ///
    /// set1.insert(set2);
    /// assert!(set1.contains(SyntaxKind::Identifier));
    /// assert!(set1.contains(SyntaxKind::Keyword));
    /// ```
    pub fn insert(&mut self, other: TokenSet) {
        self.0 |= other.0;
    }

    /// Returns a new [`TokenSet`] containing the
    /// difference of `self` and `other`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let set1 = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// let set2 = TokenSet::new(&[SyntaxKind::Keyword, SyntaxKind::Punctuation]);
    ///
    /// let difference_set = set1.difference(set2);
    /// assert!(difference_set.contains(SyntaxKind::Identifier));
    /// assert!(!difference_set.contains(SyntaxKind::Keyword));
    /// assert!(!difference_set.contains(SyntaxKind::Punctuation));
    /// ```
    pub fn difference(&self, other: TokenSet) -> TokenSet {
        TokenSet(self.0 & !other.0)
    }

    /// Returns a new [`TokenSet`] containing the
    /// symmetric difference of `self` and `other`.
    ///
    /// The symmetric difference of two sets is the set of elements which are in either of the sets
    /// but not in their intersection.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let set1 = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// let set2 = TokenSet::new(&[SyntaxKind::Keyword, SyntaxKind::Punctuation]);
    ///
    /// let symmetric_difference_set = set1.symmetric_difference(set2);
    ///
    /// assert!(symmetric_difference_set.contains(SyntaxKind::Identifier));
    /// assert!(!symmetric_difference_set.contains(SyntaxKind::Keyword));
    /// assert!(symmetric_difference_set.contains(SyntaxKind::Punctuation));
    /// ```
    pub fn symmetric_difference(&self, other: TokenSet) -> TokenSet {
        TokenSet(self.0 ^ other.0)
    }

    /// Toggles the elements of `other` in `self`.
    ///
    /// Elements present in `other` but not in `self` will be added to `self`,
    /// and elements present in both `other` and `self` will be removed from `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let mut set1 = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// let set2 = TokenSet::new(&[SyntaxKind::Keyword, SyntaxKind::Punctuation]);
    ///
    /// set1.toggle(set2);
    /// assert!(set1.contains(SyntaxKind::Identifier));
    /// assert!(!set1.contains(SyntaxKind::Keyword));
    /// assert!(set1.contains(SyntaxKind::Punctuation));
    /// ```
    pub fn toggle(&mut self, other: TokenSet) {
        self.0 ^= other.0;
    }

    /// Toggles the presence of the specified [`SyntaxKind`] in `self`.
    ///
    /// If `kind` is present in `self`, it will be removed.
    /// If `kind` is not present in `self`, it will be added.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let mut set = TokenSet::new(&[SyntaxKind::Identifier]);
    ///
    /// set.toggle_kind(SyntaxKind::Keyword);
    /// assert!(set.contains(SyntaxKind::Identifier));
    /// assert!(set.contains(SyntaxKind::Keyword));
    ///
    /// set.toggle_kind(SyntaxKind::Identifier);
    /// assert!(!set.contains(SyntaxKind::Identifier));
    /// assert!(set.contains(SyntaxKind::Keyword));
    /// ```
    pub fn toggle_kind(&mut self, kind: TokenKind) {
        self.0 ^= mask(kind.to_syntax());
    }

    /// Returns `true` if `self` is a superset of `other`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let set1 = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// let set2 = TokenSet::new(&[SyntaxKind::Identifier]);
    ///
    /// assert!(set1.is_superset(set2));
    /// assert!(!set2.is_superset(set1));
    /// ```
    pub fn is_superset(&self, other: TokenSet) -> bool {
        (self.0 & other.0) == other.0
    }

    //   /// Returns a `TokenSet` containing only the specified [`SyntaxKind`].
    //   ///
    //   /// # Example
    //   ///
    //   /// ```
    //   /// use crate::TokenSet;
    //   /// use crate::SyntaxKind;
    //   ///
    //   /// let singleton_set = TokenSet::singleton(SyntaxKind::Identifier);
    //   /// assert!(singleton_set.contains(SyntaxKind::Identifier));
    //   /// assert!(!singleton_set.contains(SyntaxKind::Keyword));
    //   /// ```
    //   pub fn singleton(kind: SyntaxKind) -> TokenSet {
    //     TokenSet(mask(kind))
    //   }

    /// Returns the complement of `self`, which includes all [`SyntaxKind`]s that are not in `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let set = TokenSet::new(&[SyntaxKind::Identifier]);
    /// let complement_set = set.complement();
    ///
    /// assert!(!complement_set.contains(SyntaxKind::Identifier));
    /// assert!(complement_set.contains(SyntaxKind::Keyword));
    /// ```
    pub fn complement(self) -> TokenSet {
        TokenSet(!self.0)
    }

    /// Returns `true` if `self` contains exactly one [`SyntaxKind`].
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let singleton_set = TokenSet::singleton(SyntaxKind::Identifier);
    /// assert!(singleton_set.is_singleton());
    ///
    /// let set = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// assert!(!set.is_singleton());
    /// ```
    ///
    /// [`SyntaxKind`]: crate::SyntaxKind
    pub fn is_singleton(self) -> bool {
        self.0 != 0 && (self.0 & (self.0 - 1)) == 0
    }

    /// Returns the number of [`SyntaxKind`]s in `self`.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let set = TokenSet::new
    /// (&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// assert_eq!(set.count(), 2);
    ///
    /// let empty_set = TokenSet::EMPTY;
    /// assert_eq!(empty_set.count(), 0);
    /// ```
    ///
    /// [`SyntaxKind`]: crate::SyntaxKind
    pub fn count(self) -> u32 {
        self.0.count_ones()
    }

    /// Removes all elements from `self` and returns a new `TokenSet` containing the removed elements.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::TokenSet;
    /// use crate::SyntaxKind;
    ///
    /// let mut set = TokenSet::new(&[SyntaxKind::Identifier, SyntaxKind::Keyword]);
    /// let taken_set = set.take();
    ///
    /// assert!(taken_set.contains(SyntaxKind::Identifier));
    /// assert!(taken_set.contains(SyntaxKind::Keyword));
    /// assert!(set.is_empty());
    /// ```
    ///
    /// [`SyntaxKind`]: crate::SyntaxKind
    pub fn take(&mut self) -> TokenSet {
        let res = *self;
        *self = TokenSet::EMPTY;
        res
    }
}

/// **Mask** for a single [`SyntaxKind`] in a `TokenSet`.
/// This operation is used to efficiently store a set of [`SyntaxKind`]s in a [`TokenSet`].
#[inline]
#[must_use]
const fn mask(kind: SyntaxKind) -> u128 {
    debug_assert!(kind as usize <= SyntaxKind::__LAST as usize, "Invalid SyntaxKind");
    1u128 << (kind as usize)
}

#[macro_export]
macro_rules! TS {
    [] => {
        TokenSet::EMPTY
    };
    [$token:tt] => {
        TokenSet::new(&[T![$token]])
    };
    [$( $token:tt ),* $(,)?] => {
        TokenSet::new(&[$( T![$token], )*])
    };
}
