// @target: es6
var v1 = {
    get [0 + 1] () {
        return 0;
    },
    set [0 + 1] (v){}
};
