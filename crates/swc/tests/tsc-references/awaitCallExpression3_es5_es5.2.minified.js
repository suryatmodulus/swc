import regeneratorRuntime from "regenerator-runtime";
function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
    try {
        var info = gen[key](arg), value = info.value;
    } catch (error) {
        reject(error);
        return;
    }
    info.done ? resolve(value) : Promise.resolve(value).then(_next, _throw);
}
function _func() {
    var fn1;
    return (_func = (fn1 = regeneratorRuntime.mark(function _callee() {
        var b;
        return regeneratorRuntime.wrap(function(_ctx) {
            for(;;)switch(_ctx.prev = _ctx.next){
                case 0:
                    return before(), _ctx.t0 = fn, _ctx.t1 = a, _ctx.next = 5, p;
                case 5:
                    _ctx.t2 = _ctx.sent, _ctx.t3 = a, b = (0, _ctx.t0)(_ctx.t1, _ctx.t2, _ctx.t3), after();
                case 9:
                case "end":
                    return _ctx.stop();
            }
        }, _callee);
    }), function() {
        var self = this, args = arguments;
        return new Promise(function(resolve, reject) {
            var gen = fn1.apply(self, args);
            function _next(value) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
            }
            function _throw(err) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
            }
            _next(void 0);
        });
    })).apply(this, arguments);
}
