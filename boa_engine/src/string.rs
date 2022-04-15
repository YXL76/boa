use crate::builtins::string::is_trimmable_whitespace;
use ::alloc::{
    alloc::{alloc, dealloc, handle_alloc_error, Layout},
    borrow::Borrow,
    boxed::Box,
    // rc::Rc,
    string::String,
    vec::Vec,
};
use arcstr::ArcStr;
use boa_gc::{unsafe_empty_trace, Finalize, Trace};
use core::{
    cell::Cell,
    hash::{Hash, Hasher},
    // marker::PhantomData,
    ops::Deref,
    ptr::{copy_nonoverlapping, NonNull},
};
use hashbrown::HashSet;
use spin::Lazy;

const CONSTANTS_ARRAY: [ArcStr; 127] = [
    // Empty string
    arcstr::literal!(""),
    // Misc
    arcstr::literal!(","),
    arcstr::literal!(":"),
    // Generic use
    arcstr::literal!("name"),
    arcstr::literal!("length"),
    arcstr::literal!("arguments"),
    arcstr::literal!("prototype"),
    arcstr::literal!("constructor"),
    // typeof
    arcstr::literal!("null"),
    arcstr::literal!("undefined"),
    arcstr::literal!("number"),
    arcstr::literal!("string"),
    arcstr::literal!("symbol"),
    arcstr::literal!("bigint"),
    arcstr::literal!("object"),
    arcstr::literal!("function"),
    // Property descriptor
    arcstr::literal!("value"),
    arcstr::literal!("get"),
    arcstr::literal!("set"),
    arcstr::literal!("writable"),
    arcstr::literal!("enumerable"),
    arcstr::literal!("configurable"),
    // Object object
    arcstr::literal!("Object"),
    arcstr::literal!("assing"),
    arcstr::literal!("create"),
    arcstr::literal!("toString"),
    arcstr::literal!("valueOf"),
    arcstr::literal!("is"),
    arcstr::literal!("seal"),
    arcstr::literal!("isSealed"),
    arcstr::literal!("freeze"),
    arcstr::literal!("isFrozen"),
    arcstr::literal!("keys"),
    arcstr::literal!("values"),
    arcstr::literal!("entries"),
    // Function object
    arcstr::literal!("Function"),
    arcstr::literal!("apply"),
    arcstr::literal!("bind"),
    arcstr::literal!("call"),
    // Array object
    arcstr::literal!("Array"),
    arcstr::literal!("from"),
    arcstr::literal!("isArray"),
    arcstr::literal!("of"),
    arcstr::literal!("get [Symbol.species]"),
    arcstr::literal!("copyWithin"),
    arcstr::literal!("entries"),
    arcstr::literal!("every"),
    arcstr::literal!("fill"),
    arcstr::literal!("filter"),
    arcstr::literal!("find"),
    arcstr::literal!("findIndex"),
    arcstr::literal!("flat"),
    arcstr::literal!("flatMap"),
    arcstr::literal!("forEach"),
    arcstr::literal!("includes"),
    arcstr::literal!("indexOf"),
    arcstr::literal!("join"),
    arcstr::literal!("map"),
    arcstr::literal!("reduce"),
    arcstr::literal!("reduceRight"),
    arcstr::literal!("reverse"),
    arcstr::literal!("shift"),
    arcstr::literal!("slice"),
    arcstr::literal!("some"),
    arcstr::literal!("sort"),
    arcstr::literal!("unshift"),
    arcstr::literal!("push"),
    arcstr::literal!("pop"),
    // String object
    arcstr::literal!("String"),
    arcstr::literal!("charAt"),
    arcstr::literal!("charCodeAt"),
    arcstr::literal!("concat"),
    arcstr::literal!("endsWith"),
    arcstr::literal!("includes"),
    arcstr::literal!("indexOf"),
    arcstr::literal!("lastIndexOf"),
    arcstr::literal!("match"),
    arcstr::literal!("matchAll"),
    arcstr::literal!("normalize"),
    arcstr::literal!("padEnd"),
    arcstr::literal!("padStart"),
    arcstr::literal!("repeat"),
    arcstr::literal!("replace"),
    arcstr::literal!("replaceAll"),
    arcstr::literal!("search"),
    arcstr::literal!("slice"),
    arcstr::literal!("split"),
    arcstr::literal!("startsWith"),
    arcstr::literal!("substring"),
    arcstr::literal!("toLowerString"),
    arcstr::literal!("toUpperString"),
    arcstr::literal!("trim"),
    arcstr::literal!("trimEnd"),
    arcstr::literal!("trimStart"),
    // Number object
    arcstr::literal!("Number"),
    // Boolean object
    arcstr::literal!("Boolean"),
    // RegExp object
    arcstr::literal!("RegExp"),
    arcstr::literal!("exec"),
    arcstr::literal!("test"),
    arcstr::literal!("flags"),
    arcstr::literal!("index"),
    arcstr::literal!("lastIndex"),
    // Symbol object
    arcstr::literal!("Symbol"),
    arcstr::literal!("for"),
    arcstr::literal!("keyFor"),
    arcstr::literal!("description"),
    arcstr::literal!("[Symbol.toPrimitive]"),
    arcstr::literal!(""),
    // Map object
    arcstr::literal!("Map"),
    arcstr::literal!("clear"),
    arcstr::literal!("delete"),
    arcstr::literal!("get"),
    arcstr::literal!("has"),
    arcstr::literal!("set"),
    arcstr::literal!("size"),
    // Set object
    arcstr::literal!("Set"),
    // Reflect object
    arcstr::literal!("Reflect"),
    // Error objects
    arcstr::literal!("Error"),
    arcstr::literal!("TypeError"),
    arcstr::literal!("RangeError"),
    arcstr::literal!("SyntaxError"),
    arcstr::literal!("ReferenceError"),
    arcstr::literal!("EvalError"),
    arcstr::literal!("URIError"),
    arcstr::literal!("message"),
    // Date object
    arcstr::literal!("Date"),
    arcstr::literal!("toJSON"),
];
// MYTODO
const MAX_CONSTANT_STRING_LENGTH: usize = 20;
/* const MAX_CONSTANT_STRING_LENGTH: usize = {
    let mut max = 0;
    let mut i = 0;
    while i < CONSTANTS_ARRAY.len() {
        let len = CONSTANTS_ARRAY[i].len();
        if len > max {
            max = len;
        }
        i += 1;
    }
    max
}; */

unsafe fn try_alloc(layout: Layout) -> *mut u8 {
    let ptr = alloc(layout);
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    ptr
}

#[repr(transparent)]
struct Constants(HashSet<JsString>);
unsafe impl Sync for Constants {}
unsafe impl Send for Constants {}

static CONSTANTS: Lazy<HashSet<JsString>> = Lazy::new(|| {
    let mut constants = HashSet::new();

    for s in CONSTANTS_ARRAY.iter() {
        let s = JsString {
            inner: s.clone(),
            // inner: Inner::new(s),
            // _marker: PhantomData,
        };
        constants.insert(s);
    }

    constants
});

/// The inner representation of a [`JsString`].
#[repr(C)]
struct Inner {
    /// The utf8 length, the number of bytes.
    len: usize,

    /// The number of references to the string.
    ///
    /// When this reaches `0` the string is deallocated.
    refcount: Cell<usize>,

    /// An empty array which is used to get the offset of string data.
    data: [u8; 0],
}

#[allow(unused)]
impl Inner {
    /// Create a new `Inner` from `&str`.
    #[inline]
    fn new(s: &str) -> NonNull<Self> {
        // We get the layout of the `Inner` type and we extend by the size
        // of the string array.
        let inner_layout = Layout::new::<Self>();
        let (layout, offset) = inner_layout
            .extend(Layout::array::<u8>(s.len()).expect("failed to create memory layout"))
            .expect("failed to extend memory layout");

        let inner = unsafe {
            let inner = try_alloc(layout).cast::<Self>();

            // Write the first part, the Inner.
            inner.write(Self {
                len: s.len(),
                refcount: Cell::new(1),
                data: [0; 0],
            });

            // Get offset into the string data.
            let data = (*inner).data.as_mut_ptr();

            debug_assert!(core::ptr::eq(inner.cast::<u8>().add(offset), data));

            // Copy string data into data offset.
            copy_nonoverlapping(s.as_ptr(), data, s.len());

            inner
        };

        // Safety: We already know it's not null, so this is safe.
        unsafe { NonNull::new_unchecked(inner) }
    }

    /// Concatenate array of strings.
    #[inline]
    fn concat_array(strings: &[&str]) -> NonNull<Self> {
        let mut total_string_size = 0;
        for string in strings {
            total_string_size += string.len();
        }

        // We get the layout of the `Inner` type and we extend by the size
        // of the string array.
        let inner_layout = Layout::new::<Self>();
        let (layout, offset) = inner_layout
            .extend(Layout::array::<u8>(total_string_size).expect("failed to create memory layout"))
            .expect("failed to extend memory layout");

        let inner = unsafe {
            let inner = try_alloc(layout).cast::<Self>();

            // Write the first part, the Inner.
            inner.write(Self {
                len: total_string_size,
                refcount: Cell::new(1),
                data: [0; 0],
            });

            // Get offset into the string data.
            let data = (*inner).data.as_mut_ptr();

            debug_assert!(core::ptr::eq(inner.cast::<u8>().add(offset), data));

            // Copy the two string data into data offset.
            let mut offset = 0;
            for string in strings {
                copy_nonoverlapping(string.as_ptr(), data.add(offset), string.len());
                offset += string.len();
            }

            inner
        };

        // Safety: We already know it's not null, so this is safe.
        unsafe { NonNull::new_unchecked(inner) }
    }

    /// Deallocate inner type with string data.
    #[inline]
    unsafe fn dealloc(x: NonNull<Self>) {
        let len = (*x.as_ptr()).len;

        let inner_layout = Layout::new::<Self>();
        let (layout, _offset) = inner_layout
            .extend(Layout::array::<u8>(len).expect("failed to create memory layout"))
            .expect("failed to extend memory layout");

        dealloc(x.as_ptr().cast::<_>(), layout);
    }
}

/// This represents a JavaScript primitive string.
///
/// This is similar to `Rc<str>`. But unlike `Rc<str>` which stores the length
/// on the stack and a pointer to the data (this is also known as fat pointers).
/// The `JsString` length and data is stored on the heap. and just an non-null
/// pointer is kept, so its size is the size of a pointer.
#[derive(Finalize, Clone)]
pub struct JsString {
    inner: ArcStr,
    // inner: NonNull<Inner>,
    // _marker: PhantomData<Rc<str>>,
}

impl Default for JsString {
    #[inline]
    fn default() -> Self {
        Self::new("")
    }
}

impl JsString {
    /// Create an empty string, same as calling default.
    #[inline]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create a new JavaScript string.
    #[inline]
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        let s = s.as_ref();

        if s.len() <= MAX_CONSTANT_STRING_LENGTH {
            if let Some(constant) = CONSTANTS.get(s).cloned() {
                return constant;
            }
        }

        Self {
            inner: ArcStr::from(s),
            // inner: Inner::new(s),
            // _marker: PhantomData,
        }
    }

    /// Concatenate two string.
    pub fn concat<T, U>(x: T, y: U) -> Self
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        let x = x.as_ref();
        let y = y.as_ref();
        Self::concat_array(&[x, y])

        /* let this = Self {
            inner: Inner::concat_array(&[x, y]),
            // _marker: PhantomData,
        };

        if this.len() <= MAX_CONSTANT_STRING_LENGTH {
            if let Some(constant) = CONSTANTS.with(|c| c.get(&this).cloned()) {
                return constant;
            }
        }

        this */
    }

    /// Concatenate array of string.
    pub fn concat_array(strings: &[&str]) -> Self {
        let this = Self {
            inner: ArcStr::from(strings.join("")),
            // _marker: PhantomData,
        };

        if this.len() <= MAX_CONSTANT_STRING_LENGTH {
            if let Some(constant) = CONSTANTS.get(&this).cloned() {
                return constant;
            }
        }

        this
    }

    /// Return the inner representation.
    /* #[inline]
    fn inner(&self) -> &Inner {
        unsafe { self.inner.as_ref() }
    } */

    /// Return the JavaScript string as a rust `&str`.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.inner.as_ref()
    }

    /// Gets the number of `JsString`s which point to this allocation.
    #[inline]
    pub fn refcount(this: &Self) -> usize {
        // MYTODO
        ArcStr::strong_count(&this.inner).unwrap()
    }

    /// Returns `true` if the two `JsString`s point to the same allocation (in a vein similar to [`ptr::eq`]).
    ///
    /// [`ptr::eq`]: core::ptr::eq
    #[inline]
    pub fn ptr_eq(x: &Self, y: &Self) -> bool {
        x.inner.as_ptr() == y.inner.as_ptr()
    }

    /// `6.1.4.1 StringIndexOf ( string, searchValue, fromIndex )`
    ///
    /// Note: Instead of returning an isize with `-1` as the "not found" value,
    /// We make use of the type system and return Option<usize> with None as the "not found" value.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-stringindexof
    pub(crate) fn index_of(&self, search_value: &Self, from_index: usize) -> Option<usize> {
        // 1. Assert: Type(string) is String.
        // 2. Assert: Type(searchValue) is String.
        // 3. Assert: fromIndex is a non-negative integer.

        // 4. Let len be the length of string.
        let len = self.encode_utf16().count();

        // 5. If searchValue is the empty String and fromIndex ≤ len, return fromIndex.
        if search_value.is_empty() && from_index <= len {
            return Some(from_index);
        }

        // 6. Let searchLen be the length of searchValue.
        let search_len = search_value.encode_utf16().count();

        // 7. For each integer i starting with fromIndex such that i ≤ len - searchLen, in ascending order, do
        for i in from_index..=len {
            if i as isize > (len as isize - search_len as isize) {
                break;
            }

            // a. Let candidate be the substring of string from i to i + searchLen.
            let candidate = String::from_utf16_lossy(
                &self
                    .encode_utf16()
                    .skip(i)
                    .take(search_len)
                    .collect::<Vec<u16>>(),
            );

            // b. If candidate is the same sequence of code units as searchValue, return i.
            if candidate == search_value.as_str() {
                return Some(i);
            }
        }

        // 8. Return -1.
        None
    }

    pub(crate) fn string_to_number(&self) -> f64 {
        let string = self.trim_matches(is_trimmable_whitespace);

        match string {
            "" => return 0.0,
            "-Infinity" => return f64::NEG_INFINITY,
            "Infinity" | "+Infinity" => return f64::INFINITY,
            _ => {}
        }

        let mut s = string.bytes();
        let base = match (s.next(), s.next()) {
            (Some(b'0'), Some(b'b' | b'B')) => Some(2),
            (Some(b'0'), Some(b'o' | b'O')) => Some(8),
            (Some(b'0'), Some(b'x' | b'X')) => Some(16),
            _ => None,
        };

        // Parse numbers that begin with `0b`, `0o` and `0x`.
        if let Some(base) = base {
            let string = &string[2..];
            if string.is_empty() {
                return f64::NAN;
            }

            // Fast path
            if let Ok(value) = u32::from_str_radix(string, base) {
                return f64::from(value);
            }

            // Slow path
            let mut value = 0.0;
            for c in s {
                if let Some(digit) = char::from(c).to_digit(base) {
                    value = value * f64::from(base) + f64::from(digit);
                } else {
                    return f64::NAN;
                }
            }
            return value;
        }

        match string {
            // Handle special cases so `fast_float` does not return infinity.
            "inf" | "+inf" | "-inf" => f64::NAN,
            string => fast_float::parse(string).unwrap_or(f64::NAN),
        }
    }
}

// Safety: [`JsString`] does not contain any objects which recquire trace,
// so this is safe.
unsafe impl Trace for JsString {
    unsafe_empty_trace!();
}

/* impl Clone for JsString {
    #[inline]
    fn clone(&self) -> Self {
        let inner = self.inner();
        inner.refcount.set(inner.refcount.get() + 1);

        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
} */

/* impl Drop for JsString {
    #[inline]
    fn drop(&mut self) {
        let inner = self.inner();
        if inner.refcount.get() == 1 {
            // Safety: If refcount is 1 and we call drop, that means this is the last
            // JsString which points to this memory allocation, so deallocating it is safe.
            unsafe {
                Inner::dealloc(self.inner);
            }
        } else {
            inner.refcount.set(inner.refcount.get() - 1);
        }
    }
} */

impl core::fmt::Debug for JsString {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl core::fmt::Display for JsString {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl From<&str> for JsString {
    #[inline]
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<Box<str>> for JsString {
    #[inline]
    fn from(s: Box<str>) -> Self {
        Self::new(s)
    }
}

impl From<String> for JsString {
    #[inline]
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl AsRef<str> for JsString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for JsString {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Deref for JsString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl PartialEq<Self> for JsString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // If they point at the same memory allocation, then they are equal.
        if Self::ptr_eq(self, other) {
            return true;
        }

        self.as_str() == other.as_str()
    }
}

impl Eq for JsString {}

impl Hash for JsString {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl PartialOrd for JsString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl Ord for JsString {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other)
    }
}

impl PartialEq<str> for JsString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<JsString> for str {
    #[inline]
    fn eq(&self, other: &JsString) -> bool {
        self == other.as_str()
    }
}

impl PartialEq<&str> for JsString {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<JsString> for &str {
    #[inline]
    fn eq(&self, other: &JsString) -> bool {
        *self == other.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::JsString;
    use core::mem::size_of;

    #[test]
    fn empty() {
        let _empty = JsString::new("");
    }

    #[test]
    fn pointer_size() {
        assert_eq!(size_of::<JsString>(), size_of::<*const u8>());
        assert_eq!(size_of::<Option<JsString>>(), size_of::<*const u8>());
    }

    #[test]
    fn refcount() {
        let x = JsString::new("Hello wrold");
        assert_eq!(JsString::refcount(&x), 1);

        {
            let y = x.clone();
            assert_eq!(JsString::refcount(&x), 2);
            assert_eq!(JsString::refcount(&y), 2);

            {
                let z = y.clone();
                assert_eq!(JsString::refcount(&x), 3);
                assert_eq!(JsString::refcount(&y), 3);
                assert_eq!(JsString::refcount(&z), 3);
            }

            assert_eq!(JsString::refcount(&x), 2);
            assert_eq!(JsString::refcount(&y), 2);
        }

        assert_eq!(JsString::refcount(&x), 1);
    }

    #[test]
    fn ptr_eq() {
        let x = JsString::new("Hello");
        let y = x.clone();

        assert!(JsString::ptr_eq(&x, &y));

        let z = JsString::new("Hello");
        assert!(!JsString::ptr_eq(&x, &z));
        assert!(!JsString::ptr_eq(&y, &z));
    }

    #[test]
    fn as_str() {
        let s = "Hello";
        let x = JsString::new(s);

        assert_eq!(x.as_str(), s);
    }

    // MYTODO
    /* #[test]
    fn hash() {
        use core::hash::{Hash, Hasher};
        use hashbrown::hash_map::DefaultHashBuilder as DefaultHasher;

        let s = "Hello, world!";
        let x = JsString::new(s);

        assert_eq!(x.as_str(), s);

        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let s_hash = hasher.finish();
        let mut hasher = DefaultHasher::new();
        x.hash(&mut hasher);
        let x_hash = hasher.finish();

        assert_eq!(s_hash, x_hash);
    } */

    #[test]
    fn concat() {
        let x = JsString::new("hello");
        let y = ", ";
        let z = JsString::new("world");
        let w = String::from("!");

        let xy = JsString::concat(x, y);
        assert_eq!(xy, "hello, ");
        assert_eq!(JsString::refcount(&xy), 1);

        let xyz = JsString::concat(xy, z);
        assert_eq!(xyz, "hello, world");
        assert_eq!(JsString::refcount(&xyz), 1);

        let xyzw = JsString::concat(xyz, w);
        assert_eq!(xyzw, "hello, world!");
        assert_eq!(JsString::refcount(&xyzw), 1);
    }
}
