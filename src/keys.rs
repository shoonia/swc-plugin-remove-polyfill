use std::matches;

pub fn function_group(obj: &str, prop: &str) -> bool {
    match obj {
        "Array" => matches!(prop, "isArray" | "from" | "of"),
        "ArrayBuffer" => prop == "isView",
        "Date" => matches!(prop, "now" | "parse" | "UTC"),
        "Object" => matches!(
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
        ),
        "Symbol" => matches!(prop, "for" | "keyFor"),
        "Promise" => matches!(
            prop,
            "all" | "race" | "reject" | "resolve" | "allSettled" | "any" | "withResolvers"
        ),
        "Proxy" => prop == "revocable",
        "Reflect" => matches!(
            prop,
            "apply"
                | "construct"
                | "defineProperty"
                | "deleteProperty"
                | "get"
                | "getOwnPropertyDescriptor"
                | "getPrototypeOf"
                | "has"
                | "isExtensible"
                | "ownKeys"
                | "preventExtensions"
                | "set"
                | "setPrototypeOf"
        ),
        _ => false,
    }
}

pub fn prototype_group(obj: &str, prop: &str) -> bool {
    match obj {
        "String" => matches!(
            prop,
            "indexOf"
                | "localeCompare"
                | "match"
                | "replace"
                | "split"
                | "substring"
                | "search"
                | "toLocaleLowerCase"
                | "toLocaleUpperCase"
                | "toLowerCase"
                | "toUpperCase"
                | "toString"
                | "valueOf"
                | "trim"
                | "normalize"
                | "includes"
                | "startsWith"
                | "endsWith"
                | "repeat"
                | "codePointAt"
                | "padStart"
                | "padEnd"
                | "trimStart"
                | "trimEnd"
                | "trimLeft"
                | "trimRight"
                | "matchAll"
                | "replaceAll"
                | "at"
                | "toWellFormed"
        ),
        "Promise" => matches!(prop, "then" | "catch" | "finally"),
        _ => false,
    }
}

pub fn well_known_symbols(obj: &str, prop: &str) -> bool {
    obj == "Symbol"
        && matches!(
            prop,
            "unscopables"
                | "iterator"
                | "toPrimitive"
                | "isConcatSpreadable"
                | "toStringTag"
                | "hasInstance"
                | "match"
                | "replace"
                | "search"
                | "split"
                | "species"
                | "asyncIterator"
                | "matchAll"
        )
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

pub fn is_built_in_member(name: &str) -> bool {
    matches!(
        name,
        "Math" | "JSON" | "Intl" | "Reflect" | "Atomics" | "globalThis"
    )
}
