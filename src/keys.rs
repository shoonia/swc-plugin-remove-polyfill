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
        "Object" => is_object_method(prop.as_ref()),
        _ => false,
    }
}
