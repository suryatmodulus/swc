// @allowJs: true
// @checkJs: true
// @target: es5
// @outDir: ./out
// @declaration: true
// @filename: index.js
module.exports = class _class {
    /**
     * @param {number} p
     */ constructor(p){
        this.t = 12 + p;
    }
};
module.exports.Sub = class _class {
    constructor(){
        this.instance = new module.exports(10);
    }
};
