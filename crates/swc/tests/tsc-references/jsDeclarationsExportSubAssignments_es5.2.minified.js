var Foo = function() {
    "use strict";
    !function(instance, Constructor) {
        if (!(instance instanceof Constructor)) throw new TypeError("Cannot call a class as a function");
    }(this, Foo);
};
module.exports = Foo, module.exports.Strings = {
    a: "A",
    b: "B"
};
