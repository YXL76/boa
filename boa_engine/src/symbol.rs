//! This module implements the global `Symbol` object.
//!
//! The data type symbol is a primitive data type.
//! The `Symbol()` function returns a value of type symbol, has static properties that expose
//! several members of built-in objects, has static methods that expose the global symbol registry,
//! and resembles a built-in object class, but is incomplete as a constructor because it does not
//! support the syntax "`new Symbol()`".
//!
//! Every symbol value returned from `Symbol()` is unique.
//!
//! More information:
//! - [MDN documentation][mdn]
//! - [ECMAScript reference][spec]
//!
//! [spec]: https://tc39.es/ecma262/#sec-symbol-value
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol

use crate::JsString;
use alloc::{string::ToString, sync::Arc};
use boa_gc::{unsafe_empty_trace, Finalize, Trace};
use core::{
    fmt::{self, Display},
    hash::{Hash, Hasher},
    sync::atomic::{AtomicU64, Ordering},
};
use spin::Once;

/// A structure that contains the JavaScript well known symbols.
///
/// # Examples
/// ```
///# use boa_engine::symbol::WellKnownSymbols;
///
/// let iterator = WellKnownSymbols::iterator();
/// assert_eq!(iterator.description().as_deref(), Some("Symbol.iterator"));
/// ```
/// This is equivalent to `let iterator = Symbol.iterator` in JavaScript.
#[derive(Debug, Clone)]
pub struct WellKnownSymbols {
    async_iterator: JsSymbol,
    has_instance: JsSymbol,
    is_concat_spreadable: JsSymbol,
    iterator: JsSymbol,
    r#match: JsSymbol,
    match_all: JsSymbol,
    replace: JsSymbol,
    search: JsSymbol,
    species: JsSymbol,
    split: JsSymbol,
    to_primitive: JsSymbol,
    to_string_tag: JsSymbol,
    unscopables: JsSymbol,
}

/// Reserved number of symbols.
///
/// This is where the well known symbol live
/// and internal engine symbols.
const RESERVED_SYMBOL_HASHES: u64 = 128;

/// Cached well known symbols
static WELL_KNOW_SYMBOLS: Once<WellKnownSymbols> = Once::new();
pub(crate) fn init() {
    WELL_KNOW_SYMBOLS.call_once(|| WellKnownSymbols::new());
}

/// Symbol hash.
///
/// For now this is an incremented u64 number.
static SYMBOL_HASH_COUNT: AtomicU64 = AtomicU64::new(RESERVED_SYMBOL_HASHES);

impl WellKnownSymbols {
    /// Create the well known symbols.
    fn new() -> Self {
        let mut count = 0;

        let async_iterator = JsSymbol::with_hash(count, Some("Symbol.asyncIterator".into()));
        count += 1;
        let has_instance = JsSymbol::with_hash(count, Some("Symbol.hasInstance".into()));
        count += 1;
        let is_concat_spreadable =
            JsSymbol::with_hash(count, Some("Symbol.isConcatSpreadable".into()));
        count += 1;
        let iterator = JsSymbol::with_hash(count, Some("Symbol.iterator".into()));
        count += 1;
        let match_ = JsSymbol::with_hash(count, Some("Symbol.match".into()));
        count += 1;
        let match_all = JsSymbol::with_hash(count, Some("Symbol.matchAll".into()));
        count += 1;
        let replace = JsSymbol::with_hash(count, Some("Symbol.replace".into()));
        count += 1;
        let search = JsSymbol::with_hash(count, Some("Symbol.search".into()));
        count += 1;
        let species = JsSymbol::with_hash(count, Some("Symbol.species".into()));
        count += 1;
        let split = JsSymbol::with_hash(count, Some("Symbol.split".into()));
        count += 1;
        let to_primitive = JsSymbol::with_hash(count, Some("Symbol.toPrimitive".into()));
        count += 1;
        let to_string_tag = JsSymbol::with_hash(count, Some("Symbol.toStringTag".into()));
        count += 1;
        let unscopables = JsSymbol::with_hash(count, Some("Symbol.unscopables".into()));

        Self {
            async_iterator,
            has_instance,
            is_concat_spreadable,
            iterator,
            r#match: match_,
            match_all,
            replace,
            search,
            species,
            split,
            to_primitive,
            to_string_tag,
            unscopables,
        }
    }

    #[inline]
    pub fn async_iterator() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }
            .async_iterator
            .clone()
    }

    /// The `Symbol.hasInstance` well known symbol.
    ///
    /// A method that determines if a `constructor` object
    /// recognizes an object as one of the `constructor`'s instances.
    /// Called by the semantics of the instanceof operator.
    #[inline]
    pub fn has_instance() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }
            .has_instance
            .clone()
    }

    /// The `Symbol.isConcatSpreadable` well known symbol.
    ///
    /// A Boolean valued property that if `true` indicates that
    /// an object should be flattened to its array elements
    /// by `Array.prototype.concat`.
    #[inline]
    pub fn is_concat_spreadable() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }
            .is_concat_spreadable
            .clone()
    }

    /// The `Symbol.iterator` well known symbol.
    ///
    /// A method that returns the default Iterator for an object.
    /// Called by the semantics of the `for-of` statement.
    #[inline]
    pub fn iterator() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }
            .iterator
            .clone()
    }

    /// The `Symbol.match` well known symbol.
    ///
    /// A regular expression method that matches the regular expression
    /// against a string. Called by the `String.prototype.match` method.
    #[inline]
    pub fn r#match() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }.r#match.clone()
    }

    /// The `Symbol.matchAll` well known symbol.
    ///
    /// A regular expression method that returns an iterator, that yields
    /// matches of the regular expression against a string.
    /// Called by the `String.prototype.matchAll` method.
    #[inline]
    pub fn match_all() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }
            .match_all
            .clone()
    }

    /// The `Symbol.replace` well known symbol.
    ///
    /// A regular expression method that replaces matched substrings
    /// of a string. Called by the `String.prototype.replace` method.
    #[inline]
    pub fn replace() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }.replace.clone()
    }

    /// The `Symbol.search` well known symbol.
    ///
    /// A regular expression method that returns the index within a
    /// string that matches the regular expression.
    /// Called by the `String.prototype.search` method.
    #[inline]
    pub fn search() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }.search.clone()
    }

    /// The `Symbol.species` well known symbol.
    ///
    /// A function valued property that is the `constructor` function
    /// that is used to create derived objects.
    #[inline]
    pub fn species() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }.species.clone()
    }

    /// The `Symbol.split` well known symbol.
    ///
    /// A regular expression method that splits a string at the indices
    /// that match the regular expression.
    /// Called by the `String.prototype.split` method.
    #[inline]
    pub fn split() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }.split.clone()
    }

    /// The `Symbol.toPrimitive` well known symbol.
    ///
    /// A method that converts an object to a corresponding primitive value.
    /// Called by the `ToPrimitive` (`Value::to_primitve`) abstract operation.
    #[inline]
    pub fn to_primitive() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }
            .to_primitive
            .clone()
    }

    /// The `Symbol.toStringTag` well known symbol.
    ///
    /// A String valued property that is used in the creation of the default
    /// string description of an object.
    /// Accessed by the built-in method `Object.prototype.toString`.
    #[inline]
    pub fn to_string_tag() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }
            .to_string_tag
            .clone()
    }

    /// The `Symbol.unscopables` well known symbol.
    ///
    /// An object valued property whose own and inherited property names are property
    /// names that are excluded from the `with` environment bindings of the associated object.
    #[inline]
    pub fn unscopables() -> JsSymbol {
        unsafe { WELL_KNOW_SYMBOLS.get_unchecked() }
            .unscopables
            .clone()
    }
}

/// The inner representation of a JavaScript symbol.
#[derive(Debug, Clone)]
struct Inner {
    hash: u64,
    description: Option<JsString>,
}

/// This represents a JavaScript symbol primitive.
#[derive(Debug, Clone)]
pub struct JsSymbol {
    inner: Arc<Inner>,
}

impl JsSymbol {
    /// Create a new symbol.
    #[inline]
    pub fn new(description: Option<JsString>) -> Self {
        let hash = SYMBOL_HASH_COUNT
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |h| Some(h + 1))
            .unwrap();

        Self {
            inner: Arc::new(Inner { hash, description }),
        }
    }

    /// Create a new symbol with a specified hash and description.
    #[inline]
    fn with_hash(hash: u64, description: Option<JsString>) -> Self {
        Self {
            inner: Arc::new(Inner { hash, description }),
        }
    }

    /// Returns the `Symbol`s description.
    #[inline]
    pub fn description(&self) -> Option<JsString> {
        self.inner.description.clone()
    }

    /// Returns the `Symbol`s hash.
    ///
    /// The hash is guaranteed to be unique.
    #[inline]
    pub fn hash(&self) -> u64 {
        self.inner.hash
    }

    /// Abstract operation `SymbolDescriptiveString ( sym )`
    ///
    /// More info:
    /// - [ECMAScript reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-symboldescriptivestring
    pub fn descriptive_string(&self) -> JsString {
        self.to_string().into()
    }
}

impl Finalize for JsSymbol {}

// Safety: `JsSymbol` does not contain any object that require trace,
// so this is safe.
unsafe impl Trace for JsSymbol {
    unsafe_empty_trace!();
}

impl Display for JsSymbol {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner.description {
            Some(desc) => write!(f, "Symbol({desc})"),
            None => write!(f, "Symbol()"),
        }
    }
}

impl Eq for JsSymbol {}

impl PartialEq for JsSymbol {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.hash == other.inner.hash
    }
}

impl PartialOrd for JsSymbol {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.inner.hash.partial_cmp(&other.inner.hash)
    }
}

impl Ord for JsSymbol {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.inner.hash.cmp(&other.inner.hash)
    }
}

impl Hash for JsSymbol {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash.hash(state);
    }
}
