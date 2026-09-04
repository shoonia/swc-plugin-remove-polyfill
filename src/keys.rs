use std::matches;

#[inline(always)]
fn is_object_method(prop: &str) -> bool {
    matches!(
        prop,
        "create"
            | "keys"
            | "getPrototypeOf"
            | "defineProperty"
            | "defineProperties"
            | "getOwnPropertyDescriptor"
            | "getOwnPropertyNames"
            | "freeze"
            | "isFrozen"
            | "seal"
            | "isSealed"
            | "isExtensible"
            | "preventExtensions"
            | "is"
            | "setPrototypeOf"
            | "getOwnPropertySymbols"
            | "assign"
            | "values"
            | "entries"
            | "getOwnPropertyDescriptors"
            | "fromEntries"
            | "hasOwn"
    )
}

pub fn function_group(obj: &str, prop: &str) -> bool {
    match obj.as_ref() {
        "Array" => matches!(prop.as_ref(), "isArray" | "from" | "of"),
        "ArrayBuffer" => prop == "isView",
        "Date" => matches!(prop.as_ref(), "now" | "parse" | "UTC"),
        "Object" => is_object_method(prop.as_ref()),
        "Symbol" => matches!(prop.as_ref(), "for" | "keyFor"),
        _ => false,
    }
}

pub fn is_built_in_constructor(name: &str) -> bool {
    matches!(
        name,
        "Blob"
            | "ArrayBuffer"
            | "Int8Array"
            | "Uint8Array"
            | "Uint8ClampedArray"
            | "Int16Array"
            | "Uint16Array"
            | "Int32Array"
            | "Uint32Array"
            | "Float32Array"
            | "Float64Array"
            | "DataView"
            | "URL"
            | "Promise"
            | "WeakMap"
            | "WeakSet"
            | "Set"
            | "Map"
            | "Symbol"
            | "Proxy"
            | "URLSearchParams"
            | "BigInt"
            | "BigInt64Array"
            | "BigUint64Array"
            | "WeakRef"
            | "FinalizationRegistry"
            | "AggregateError"
    )
}
