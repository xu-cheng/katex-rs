var global;
try {
    global = Function('return this')();
} catch (e) {
    global = window;
}
global.katexRenderToString = function(input, opts) {
    return katex.renderToString(input, opts);
};
