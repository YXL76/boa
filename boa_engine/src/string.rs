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
use spin::Once;

// MYTODO
const MAX_CONSTANT_STRING_LENGTH: usize = 16;
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

static CONSTANTS: Once<HashSet<JsString>> = Once::new();

pub(crate) fn init() {
    CONSTANTS.call_once(|| {
        let mut constants = HashSet::new();

        constants.insert(JsString {
            inner: arcstr::literal!(""),
        });
        constants.insert(JsString {
            inner: arcstr::literal!(","),
        });
        constants.insert(JsString {
            inner: arcstr::literal!(":"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("name"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("length"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("arguments"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("prototype"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("constructor"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("return"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("throw"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("global"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("globalThis"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("null"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("undefined"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("number"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("string"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("symbol"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("bigint"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("object"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("function"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("value"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("set"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("writable"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("enumerable"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("configurable"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Object"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("assign"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("create"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toString"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("valueOf"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("is"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("seal"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("isSealed"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("freeze"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("isFrozen"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("isExtensible"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("hasOwnProperty"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("isPrototypeOf"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setPrototypeOf"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getPrototypeOf"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("defineProperty"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("defineProperties"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("deleteProperty"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("construct"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("hasOwn"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("ownKeys"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("keys"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("values"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("entries"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("fromEntries"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Function"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("apply"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("bind"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("call"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Generator"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("at"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("from"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("isArray"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("of"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("copyWithin"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("entries"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("every"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("fill"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("filter"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("find"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("findIndex"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("findLast"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("findLastIndex"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("flat"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("flatMap"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("forEach"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("includes"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("indexOf"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("join"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("map"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("next"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("reduce"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("reduceRight"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("reverse"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("shift"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("slice"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("splice"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("some"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("sort"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("unshift"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("push"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("pop"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("String"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("charAt"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("charCodeAt"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("codePointAt"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("concat"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("endsWith"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("fromCharCode"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("fromCodePoint"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("includes"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("indexOf"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("lastIndexOf"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("match"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("matchAll"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("normalize"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("padEnd"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("padStart"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("raw"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("repeat"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("replace"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("replaceAll"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("search"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("slice"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("split"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("startsWith"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("substr"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("substring"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toLocaleString"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toLowerCase"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toUpperCase"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("trim"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("trimEnd"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("trimStart"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Number"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Infinity"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("NaN"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("parseInt"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("parseFloat"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("isFinite"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("isNaN"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("parseInt"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("EPSILON"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("MAX_SAFE_INTEGER"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("MIN_SAFE_INTEGER"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("MAX_VALUE"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("MIN_VALUE"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("isSafeInteger"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("isInteger"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toExponential"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toFixed"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toPrecision"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Boolean"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("BigInt"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("asIntN"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("asUintN"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("RegExp"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("exec"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("test"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("flags"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("index"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("lastIndex"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("hasIndices"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("ignoreCase"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("multiline"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("dotAll"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("unicode"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("sticky"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("source"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get hasIndices"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get global"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get ignoreCase"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get multiline"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get dotAll"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get unicode"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get sticky"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get flags"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get source"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Symbol"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("for"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("keyFor"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("description"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("asyncIterator"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("hasInstance"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("species"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Symbol.species"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("unscopables"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("iterator"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Symbol.iterator"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Symbol.match"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("[Symbol.match]"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Symbol.matchAll"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Symbol.replace"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("[Symbol.replace]"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Symbol.search"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("[Symbol.search]"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Symbol.split"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("[Symbol.split]"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toStringTag"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toPrimitive"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get description"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Map"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("clear"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("delete"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("has"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("set"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("size"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Set"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("add"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Reflect"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Proxy"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("revocable"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Error"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("AggregateError"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("TypeError"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("RangeError"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("SyntaxError"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("ReferenceError"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("EvalError"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("ThrowTypeError"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("URIError"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("message"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Date"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toJSON"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getDate"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getDay"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getFullYear"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getHours"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getMilliseconds"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getMinutes"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getMonth"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getSeconds"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getTime"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getYear"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getUTCDate"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getUTCDay"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getUTCFullYear"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getUTCHours"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getUTCMinutes"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getUTCMonth"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getUTCSeconds"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setDate"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setFullYear"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setHours"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setMilliseconds"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setMinutes"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setMonth"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setSeconds"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setYear"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setTime"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setUTCDate"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setUTCFullYear"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setUTCHours"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setUTCMinutes"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setUTCMonth"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setUTCSeconds"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toDateString"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toGMTString"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toISOString"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toTimeString"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("toUTCString"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("now"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("UTC"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("JSON"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("parse"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("stringify"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Array Iterator"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Set Iterator"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("String Iterator"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Map Iterator"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("For In Iterator"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Math"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("LN10"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("LN2"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("LOG10E"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("LOG2E"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("PI"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("SQRT1_2"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("SQRT2"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("abs"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("acos"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("acosh"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("asin"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("asinh"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("atan"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("atanh"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("atan2"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("cbrt"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("ceil"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("clz32"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("cos"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("cosh"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("exp"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("expm1"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("floor"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("fround"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("hypot"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("imul"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("log"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("log1p"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("log10"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("log2"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("max"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("min"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("pow"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("random"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("round"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("sign"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("sin"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("sinh"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("sqrt"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("tan"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("tanh"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("trunc"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Intl"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("DateTimeFormat"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("TypedArray"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("ArrayBuffer"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Int8Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Uint8Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Int16Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Uint16Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Int32Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Uint32Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("BigInt64Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("BigUint64Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Float32Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Float64Array"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("buffer"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("byteLength"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("byteOffset"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("isView"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("subarray"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get byteLength"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get buffer"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get byteOffset"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get size"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("get length"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("DataView"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getBigInt64"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getBigUint64"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getFloat32"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getFloat64"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getInt8"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getInt16"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getInt32"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getUint8"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getUint16"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("getUint32"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setBigInt64"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setBigUint64"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setFloat32"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setFloat64"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setInt8"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setInt16"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setInt32"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setUint8"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setUint16"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("setUint32"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("console"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("assert"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("debug"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("error"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("info"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("trace"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("warn"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("exception"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("count"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("countReset"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("group"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("groupCollapsed"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("groupEnd"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("time"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("timeLog"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("timeEnd"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("dir"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("dirxml"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("a"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("b"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("c"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("d"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("e"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("f"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("g"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("h"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("i"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("j"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("k"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("l"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("m"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("n"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("o"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("p"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("q"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("r"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("s"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("t"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("u"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("v"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("w"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("x"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("y"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("z"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("A"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("B"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("C"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("D"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("E"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("F"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("G"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("H"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("I"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("J"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("K"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("L"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("M"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("N"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("O"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("P"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Q"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("R"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("S"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("T"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("U"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("V"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("W"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("X"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Y"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("Z"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("_"),
        });
        constants.insert(JsString {
            inner: arcstr::literal!("$"),
        });

        constants
    });
}

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

    #[inline]
    fn as_str(&self) -> &str {
        unsafe {
            let slice = core::slice::from_raw_parts(self.data.as_ptr(), self.len);
            core::str::from_utf8_unchecked(slice)
        }
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
            if let Some(constant) = unsafe { CONSTANTS.get_unchecked() }.get(s).cloned() {
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

        if s.len() <= MAX_CONSTANT_STRING_LENGTH {
            if let Some(constant) = CONSTANTS.with(|c| c.get(s).cloned()) {
                unsafe { Inner::dealloc(inner) };
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
            if let Some(constant) = unsafe { CONSTANTS.get_unchecked() }.get(&this).cloned() {
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
    pub fn refcount(this: &Self) -> Option<usize> {
        // MYTODO
        ArcStr::strong_count(&this.inner)
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
        if let InnerKind::Heap(inner) = self.inner() {
            inner.refcount.set(inner.refcount.get() + 1);
        }
        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
} */

/* impl Drop for JsString {
    #[inline]
    fn drop(&mut self) {
        if let InnerKind::Heap(inner) = self.inner() {
            if inner.refcount.get() == 1 {
                // Safety: If refcount is 1 and we call drop, that means this is the last
                // JsString which points to this memory allocation, so deallocating it is safe.
                unsafe {
                    Inner::dealloc(self.inner.get_heap_unchecked());
                }
            } else {
                inner.refcount.set(inner.refcount.get() - 1);
            }
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
        assert_eq!(JsString::refcount(&x), Some(1));

        {
            let y = x.clone();
            assert_eq!(JsString::refcount(&x), Some(2));
            assert_eq!(JsString::refcount(&y), Some(2));

            {
                let z = y.clone();
                assert_eq!(JsString::refcount(&x), Some(3));
                assert_eq!(JsString::refcount(&y), Some(3));
                assert_eq!(JsString::refcount(&z), Some(3));
            }

            assert_eq!(JsString::refcount(&x), Some(2));
            assert_eq!(JsString::refcount(&y), Some(2));
        }

        assert_eq!(JsString::refcount(&x), Some(1));
    }

    #[test]
    fn static_refcount() {
        let x = JsString::new("");
        assert_eq!(JsString::refcount(&x), None);

        {
            let y = x.clone();
            assert_eq!(JsString::refcount(&x), None);
            assert_eq!(JsString::refcount(&y), None);
        };

        assert_eq!(JsString::refcount(&x), None);
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
    fn static_ptr_eq() {
        let x = JsString::new("");
        let y = x.clone();

        assert!(JsString::ptr_eq(&x, &y));

        let z = JsString::new("");
        assert!(JsString::ptr_eq(&x, &z));
        assert!(JsString::ptr_eq(&y, &z));
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
        assert_eq!(JsString::refcount(&xy), Some(1));

        let xyz = JsString::concat(xy, z);
        assert_eq!(xyz, "hello, world");
        assert_eq!(JsString::refcount(&xyz), Some(1));

        let xyzw = JsString::concat(xyz, w);
        assert_eq!(xyzw, "hello, world!");
        assert_eq!(JsString::refcount(&xyzw), Some(1));
    }
}
