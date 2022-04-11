//! This module implements the global `Reflect` object.
//!
//! The `Reflect` global object is a built-in object that provides methods for interceptable
//! JavaScript operations.
//!
//! More information:
//!  - [ECMAScript reference][spec]
//!  - [MDN documentation][mdn]
//!
//! [spec]: https://tc39.es/ecma262/#sec-reflect-object
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect

use super::{Array, JsArgs};
use crate::{
    builtins::{self, BuiltIn},
    object::{JsObject, ObjectInitializer},
    property::Attribute,
    symbol::WellKnownSymbols,
    Context, JsResult, JsValue,
};
use alloc::vec::Vec;
use boa_profiler::Profiler;
use tap::{Conv, Pipe};

#[cfg(test)]
mod tests;

/// Javascript `Reflect` object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Reflect;

impl BuiltIn for Reflect {
    const NAME: &'static str = "Reflect";

    fn init(context: &mut Context) -> Option<JsValue> {
        let _timer = Profiler::global().start_event(Self::NAME, "init");

        let to_string_tag = WellKnownSymbols::to_string_tag();

        ObjectInitializer::new(context)
            .function(Self::apply, "apply", 3)
            .function(Self::construct, "construct", 2)
            .function(Self::define_property, "defineProperty", 3)
            .function(Self::delete_property, "deleteProperty", 2)
            .function(Self::get, "get", 2)
            .function(
                Self::get_own_property_descriptor,
                "getOwnPropertyDescriptor",
                2,
            )
            .function(Self::get_prototype_of, "getPrototypeOf", 1)
            .function(Self::has, "has", 2)
            .function(Self::is_extensible, "isExtensible", 1)
            .function(Self::own_keys, "ownKeys", 1)
            .function(Self::prevent_extensions, "preventExtensions", 1)
            .function(Self::set, "set", 3)
            .function(Self::set_prototype_of, "setPrototypeOf", 2)
            .property(
                to_string_tag,
                Self::NAME,
                Attribute::READONLY | Attribute::NON_ENUMERABLE | Attribute::CONFIGURABLE,
            )
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }
}

impl Reflect {
    /// Calls a target function with arguments.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.apply
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/apply
    pub(crate) fn apply(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be a function"))?;
        let this_arg = args.get_or_undefined(1);
        let args_list = args.get_or_undefined(2);

        if !target.is_callable() {
            return context.throw_type_error("target must be a function");
        }
        let args = args_list.create_list_from_array_like(&[], context)?;
        target.call(this_arg, &args, context)
    }

    /// Calls a target function as a constructor with arguments.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.construct
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/construct
    pub(crate) fn construct(
        _: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be a function"))?;
        let args_list = args.get_or_undefined(1);

        if !target.is_constructor() {
            return context.throw_type_error("target must be a constructor");
        }

        let new_target = if let Some(new_target) = args.get(2) {
            if new_target.as_object().map(JsObject::is_constructor) != Some(true) {
                return context.throw_type_error("newTarget must be constructor");
            }
            new_target.clone()
        } else {
            target.clone().into()
        };

        let args = args_list.create_list_from_array_like(&[], context)?;
        target.construct(&args, &new_target, context)
    }

    /// Defines a property on an object.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.defineProperty
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/defineProperty
    pub(crate) fn define_property(
        _: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be an object"))?;
        let key = args.get_or_undefined(1).to_property_key(context)?;
        let prop_desc: JsValue = args
            .get(2)
            .and_then(|v| v.as_object().cloned())
            .ok_or_else(|| context.construct_type_error("property descriptor must be an object"))?
            .into();

        target
            .__define_own_property__(key, prop_desc.to_property_descriptor(context)?, context)
            .map(Into::into)
    }

    /// Defines a property on an object.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.deleteproperty
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/deleteProperty
    pub(crate) fn delete_property(
        _: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be an object"))?;
        let key = args.get_or_undefined(1).to_property_key(context)?;

        Ok(target.__delete__(&key, context)?.into())
    }

    /// Gets a property of an object.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.get
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/get
    pub(crate) fn get(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        // 1. If Type(target) is not Object, throw a TypeError exception.
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be an object"))?;
        // 2. Let key be ? ToPropertyKey(propertyKey).
        let key = args.get_or_undefined(1).to_property_key(context)?;
        // 3. If receiver is not present, then
        let receiver = if let Some(receiver) = args.get(2).cloned() {
            receiver
        } else {
            // 3.a. Set receiver to target.
            target.clone().into()
        };
        // 4. Return ? target.[[Get]](key, receiver).
        target.__get__(&key, receiver, context)
    }

    /// Gets a property of an object.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.getownpropertydescriptor
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getOwnPropertyDescriptor
    pub(crate) fn get_own_property_descriptor(
        _: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        if args.get_or_undefined(0).is_object() {
            // This function is the same as Object.prototype.getOwnPropertyDescriptor, that why
            // it is invoked here.
            builtins::object::Object::get_own_property_descriptor(
                &JsValue::undefined(),
                args,
                context,
            )
        } else {
            context.throw_type_error("target must be an object")
        }
    }

    /// Gets the prototype of an object.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.getprototypeof
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/getPrototypeOf
    pub(crate) fn get_prototype_of(
        _: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be an object"))?;
        Ok(target
            .__get_prototype_of__(context)?
            .map_or(JsValue::Null, JsValue::new))
    }

    /// Returns `true` if the object has the property, `false` otherwise.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.has
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/has
    pub(crate) fn has(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be an object"))?;
        let key = args
            .get(1)
            .unwrap_or(&JsValue::undefined())
            .to_property_key(context)?;
        Ok(target.__has_property__(&key, context)?.into())
    }

    /// Returns `true` if the object is extensible, `false` otherwise.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.isextensible
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/isExtensible
    pub(crate) fn is_extensible(
        _: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be an object"))?;
        Ok(target.__is_extensible__(context)?.into())
    }

    /// Returns an array of object own property keys.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.ownkeys
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/ownKeys
    pub(crate) fn own_keys(
        _: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be an object"))?;

        let keys: Vec<JsValue> = target
            .__own_property_keys__(context)?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(Array::create_array_from_list(keys, context).into())
    }

    /// Prevents new properties from ever being added to an object.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.preventextensions
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/preventExtensions
    pub(crate) fn prevent_extensions(
        _: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be an object"))?;

        Ok(target.__prevent_extensions__(context)?.into())
    }

    /// Sets a property of an object.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.set
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/set
    pub(crate) fn set(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be an object"))?;
        let key = args.get_or_undefined(1).to_property_key(context)?;
        let value = args.get_or_undefined(2);
        let receiver = if let Some(receiver) = args.get(3).cloned() {
            receiver
        } else {
            target.clone().into()
        };
        Ok(target
            .__set__(key, value.clone(), receiver, context)?
            .into())
    }

    /// Sets the prototype of an object.
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-reflect.setprototypeof
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect/setPrototypeOf
    pub(crate) fn set_prototype_of(
        _: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let target = args
            .get(0)
            .and_then(JsValue::as_object)
            .ok_or_else(|| context.construct_type_error("target must be an object"))?;
        let proto = match args.get_or_undefined(1) {
            JsValue::Object(obj) => Some(obj.clone()),
            JsValue::Null => None,
            _ => return context.throw_type_error("proto must be an object or null"),
        };
        Ok(target.__set_prototype_of__(proto, context)?.into())
    }
}
