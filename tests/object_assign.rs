mod common;

use common::run_test;

#[test]
fn object_assign_basic_or_polyfill() {
    run_test(
        r#"
        var assign = Object.assign || function (e) {
          for (var t = 1; t < arguments.length; t++) {
            var n = arguments[t];

            for (var r in n) {
              if (Object.prototype.hasOwnProperty.call(n, r)) {
                e[r] = n[r]
              }
            }
          }
          return e;
        };
        "#,
        r#"
        var assign = Object.assign;
        "#,
    );
}

#[test]
fn object_assign_ts_helper_reassignment() {
    run_test(
        r#"
        var __assign = function () {
          __assign = Object.assign || function __assign(t) {
            for (var s, i = 1, n = arguments.length; i < n; i++) {
              s = arguments[i];
              for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p)) t[p] = s[p];
            }
            return t;
          }
          return __assign.apply(this, arguments);
        };
        "#,
        r#"
        var __assign = function() {
            __assign = Object.assign;
            return __assign.apply(this, arguments);
        };
        "#,
    );
}

#[test]
fn object_assign_this_helper() {
    run_test(
        r#"
        var i = this && this.__assign || function () {
          return i = Object.assign || function (e) {
            for (var t, n = 1, r = arguments.length; n < r; n++)
              for (var o in t = arguments[n])
                Object.prototype.hasOwnProperty.call(t, o) && (e[o] = t[o]);
            return e;
          },
            i.apply(this, arguments);
        };
        "#,
        r#"
        var i = this && this.__assign || function() {
            return i = Object.assign, i.apply(this, arguments);
        };
        "#,
    );
}

#[test]
fn object_assign_iife_return() {
    run_test(
        r#"
        var c = function () {
          return (c = Object.assign || function (t) {
            for (var n, r = 1, e = arguments.length; r < e; r++)
              for (var i in n = arguments[r])
                Object.prototype.hasOwnProperty.call(n, i) && (t[i] = n[i]);
            return t
          }).apply(this, arguments)
        };
        "#,
        r#"
        var c = function() {
            return (c = Object.assign).apply(this, arguments);
        };
        "#,
    );
}

#[test]
fn object_assign_ternary_bind() {
    run_test(
        r#"
        function a() {
          return a = Object.assign ? Object.assign.bind() : function (e) {
            for (var t = 1; t < arguments.length; t++) {
              var n = arguments[t];
              for (var r in n)
                Object.prototype.hasOwnProperty.call(n, r) && (e[r] = n[r])
            }
            return e
          }, a.apply(this, arguments)
        }
        "#,
        r#"
        function a() {
            return a = Object.assign.bind(), a.apply(this, arguments);
        }
        "#,
    );
}

#[test]
fn object_assign_typeof_ternary() {
    run_test(
        r#"
        var pa = "function" == typeof Object.assign ? Object.assign : function (a, b) {
          for (var c = 1; c < arguments.length; c++) {
            var d = arguments[c];
            if (d) for (var e in d) Object.prototype.hasOwnProperty.call(d, e) && (a[e] = d[e])
          }
          return a
        }
        "#,
        r#"
        var pa = "function" == typeof Object.assign ? Object.assign : function(a, b) {
            for(var c = 1; c < arguments.length; c++){
                var d = arguments[c];
                if (d) for(var e in d)Object.prototype.hasOwnProperty.call(d, e) && (a[e] = d[e]);
            }
            return a;
        };
        "#,
    );
}

#[test]
fn object_assign_this_or_global_or_polyfill() {
    run_test(
        r#"
        var __assign = (this && this.__assign) || Object.assign || function (t) {
          for (var s, i = 1, n = arguments.length; i < n; i++) {
            s = arguments[i];
            for (var p in s) if (Object.prototype.hasOwnProperty.call(s, p)) t[p] = s[p];
          }
          return t;
        };
        "#,
        r#"
        var __assign = this && this.__assign || Object.assign || function(t) {
            for(var s, i = 1, n = arguments.length; i < n; i++){
                s = arguments[i];
                for(var p in s)if (Object.prototype.hasOwnProperty.call(s, p)) t[p] = s[p];
            }
            return t;
        };
        "#,
    );
}

#[test]
fn object_assign_define_property_guard() {
    run_test(
        r#"
        "function" != typeof Object.assign && Object.defineProperty(Object, "assign", {
          value: function (e, t) {
            if (null == e)
              throw new TypeError("Cannot convert undefined or null to object");
            for (var o = Object(e), n = 1; n < arguments.length; n++) {
              var r = arguments[n];
              if (null != r)
                for (var i in r)
                  Object.prototype.hasOwnProperty.call(r, i) && (o[i] = r[i])
            }
            return o
          },
          writable: !0,
          configurable: !0
        });
        "#,
        r#"
        "function" != typeof Object.assign && Object.defineProperty(Object, "assign", {
            value: function(e, t) {
                if (null == e) throw new TypeError("Cannot convert undefined or null to object");
                for(var o = Object(e), n = 1; n < arguments.length; n++){
                    var r = arguments[n];
                    if (null != r) for(var i in r)Object.prototype.hasOwnProperty.call(r, i) && (o[i] = r[i]);
                }
                return o;
            },
            writable: !0,
            configurable: !0
        });
        "#,
    );
}

#[test]
fn object_assign_shadowed_not_touched() {
    run_test(
        r#"
    function foo() {
      let Object = {};
      Object.assign || polyfillObjectAssign({}, x);
    }
    "#,
        r#"
    function foo() {
      let Object = {};
      Object.assign || polyfillObjectAssign({}, x);
    }
    "#,
    )
}
