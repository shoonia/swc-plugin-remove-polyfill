use std::matches;

pub fn is_static_method(obj: &str, prop: &str) -> bool {
    match obj {
        "Array" => matches!(prop, "from" | "fromAsync" | "isArray" | "of"),
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
                | "groupBy"
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
        "Number" => matches!(
            prop,
            "isFinite" | "isInteger" | "isNaN" | "isSafeInteger" | "parseFloat" | "parseInt"
        ),
        "String" => matches!(prop, "fromCharCode" | "fromCodePoint" | "raw"),
        "Math" => matches!(
            prop,
            "abs"
                | "acos"
                | "asin"
                | "atan"
                | "atan2"
                | "ceil"
                | "cos"
                | "exp"
                | "floor"
                | "log"
                | "max"
                | "min"
                | "pow"
                | "random"
                | "round"
                | "sin"
                | "sqrt"
                | "tan"
                | "imul"
                | "acosh"
                | "asinh"
                | "atanh"
                | "cbrt"
                | "clz32"
                | "cosh"
                | "expm1"
                | "fround"
                | "hypot"
                | "log10"
                | "log1p"
                | "log2"
                | "sign"
                | "sinh"
                | "tanh"
                | "trunc"
        ),
        "JSON" => matches!(prop, "parse" | "stringify"),
        "Error" => prop == "captureStackTrace",
        "URL" => matches!(
            prop,
            "canParse" | "createObjectURL" | "parse" | "revokeObjectURL"
        ),
        _ => false,
    }
}

fn is_array_like_prototype_method(prop: &str) -> bool {
    matches!(
        prop,
        "toString"
            | "join"
            | "reverse"
            | "slice"
            | "sort"
            | "indexOf"
            | "lastIndexOf"
            | "every"
            | "some"
            | "forEach"
            | "map"
            | "filter"
            | "reduce"
            | "reduceRight"
            | "find"
            | "findIndex"
            | "fill"
            | "copyWithin"
            | "entries"
            | "keys"
            | "values"
            | "at"
            | "includes"
            | "findLast"
            | "findLastIndex"
            | "toReversed"
            | "toSorted"
            | "with"
    )
}

pub fn is_prototype_method(obj: &str, prop: &str) -> bool {
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
        "Array" => {
            matches!(
                prop,
                "concat"
                    | "shift"
                    | "unshift"
                    | "splice"
                    | "pop"
                    | "push"
                    | "toLocaleString"
                    | "flat"
                    | "flatMap"
                    | "toSpliced"
            ) || is_array_like_prototype_method(prop)
        }
        "Int8Array" | "Int16Array" | "Int32Array" | "Uint8Array" | "Uint16Array"
        | "Uint32Array" | "Uint8ClampedArray" | "Float32Array" | "Float64Array"
        | "BigInt64Array" | "BigUint64Array" => is_array_like_prototype_method(prop),
        "ArrayBuffer" => prop == "slice",
        "Function" => prop == "bind",
        "Blob" => matches!(prop, "slice" | "arrayBuffer" | "stream" | "text"),
        "DataView" => matches!(
            prop,
            "getInt8"
                | "setInt8"
                | "getInt16"
                | "setInt16"
                | "getInt32"
                | "setInt32"
                | "getUint8"
                | "setUint8"
                | "getUint16"
                | "setUint16"
                | "getUint32"
                | "setUint32"
                | "getFloat32"
                | "setFloat32"
                | "getFloat64"
                | "setFloat64"
                | "getBigInt64"
                | "setBigInt64"
                | "getBigUint64"
                | "setBigUint64"
                | "getFloat16"
                | "setFloat16"
        ),
        _ => false,
    }
}

pub fn is_well_known_symbol(obj: &str, prop: &str) -> bool {
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
