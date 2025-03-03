function _extends() {
    _extends = Object.assign || function(target) {
        for(var i = 1; i < arguments.length; i++){
            var source = arguments[i];
            for(var key in source){
                if (Object.prototype.hasOwnProperty.call(source, key)) {
                    target[key] = source[key];
                }
            }
        }
        return target;
    };
    return _extends.apply(this, arguments);
}
// @filename: file.tsx
// @jsx: preserve
// @module: amd
// @noLib: true
// @skipLibCheck: true
// @libFiles: react.d.ts,lib.d.ts
var React = require('react');
function createComponent(arg) {
    var a1 = /*#__PURE__*/ React.createElement(Component, _extends({}, arg));
    var a2 = /*#__PURE__*/ React.createElement(Component, _extends({}, arg, {
        prop1: true
    }));
}
function Bar(arg) {
    var a1 = /*#__PURE__*/ React.createElement(ComponentSpecific, _extends({}, arg, {
        "ignore-prop": "hi"
    })); // U is number
    var a2 = /*#__PURE__*/ React.createElement(ComponentSpecific1, _extends({}, arg, {
        "ignore-prop": 10
    })); // U is number
    var a3 = /*#__PURE__*/ React.createElement(ComponentSpecific, _extends({}, arg, {
        prop: "hello"
    })); // U is "hello"
    var a4 = /*#__PURE__*/ React.createElement(ComponentSpecific, _extends({}, arg, {
        prop1: "hello"
    })); // U is "hello"
}
export { };
