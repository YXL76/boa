use crate::{
    object::{JsObject, JsObjectType},
    Context, JsResult, JsValue,
};
use boa_gc::{Finalize, Trace};
use std::ops::Deref;

/// JavaScript `Function` rust object.
#[derive(Debug, Clone, Trace, Finalize)]
pub struct JsFunction {
    inner: JsObject,
}

impl JsFunction {
    #[inline]
    pub(crate) fn from_object_unchecked(object: JsObject) -> Self {
        Self { inner: object }
    }

    #[inline]
    pub fn from_object(object: JsObject, context: &mut Context) -> JsResult<Self> {
        if object.borrow().is_function() {
            Ok(Self::from_object_unchecked(object))
        } else {
            context.throw_type_error("object is not an Function")
        }
    }
}

impl From<JsFunction> for JsObject {
    #[inline]
    fn from(o: JsFunction) -> Self {
        o.inner.clone()
    }
}

impl From<JsFunction> for JsValue {
    #[inline]
    fn from(o: JsFunction) -> Self {
        o.inner.clone().into()
    }
}

impl Deref for JsFunction {
    type Target = JsObject;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl JsObjectType for JsFunction {}
