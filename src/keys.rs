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
        "Object" => is_object_method(prop.as_ref()),
        "Symbol" => matches!(prop.as_ref(), "for" | "keyFor"),
        _ => false,
    }
}
