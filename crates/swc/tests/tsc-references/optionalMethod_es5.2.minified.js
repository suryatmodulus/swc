function _defineProperties(target, props) {
    for(var i = 0; i < props.length; i++){
        var descriptor = props[i];
        descriptor.enumerable = descriptor.enumerable || !1, descriptor.configurable = !0, "value" in descriptor && (descriptor.writable = !0), Object.defineProperty(target, descriptor.key, descriptor);
    }
}
var Base = function() {
    "use strict";
    var Constructor, protoProps, staticProps;
    function Base() {
        !function(instance, Constructor) {
            if (!(instance instanceof Constructor)) throw new TypeError("Cannot call a class as a function");
        }(this, Base);
    }
    return protoProps = [
        {
            key: "method",
            value: function() {}
        }
    ], _defineProperties((Constructor = Base).prototype, protoProps), staticProps && _defineProperties(Constructor, staticProps), Base;
}();
