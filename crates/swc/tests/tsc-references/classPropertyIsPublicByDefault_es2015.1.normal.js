class C {
    get y() {
        return null;
    }
    set y(x) {}
    foo() {}
    static get b() {
        return null;
    }
    static set b(x) {}
    static foo() {}
}
var c;
c.x;
c.y;
c.y = 1;
c.foo();
C.a;
C.b();
C.b = 1;
C.foo();
