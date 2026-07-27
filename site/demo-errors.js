// Forwards the demo's first fatal boot error to the parent docs page, which
// swaps the example card into its error state. Mobile browsers in particular
// expose `navigator.gpu` yet fail later — null adapter for the GPU, shared
// wasm memory limits, executable-memory exhaustion — and without this the
// stage just stays silently blank. Loaded synchronously in the shell's head so
// it observes failures inside trunk's wasm bootstrap.
(function () {
    if (window.parent === window) return;
    var sent = false;
    function report(reason) {
        if (sent) return;
        sent = true;
        var message =
            reason && reason.message ? reason.message : String(reason || "unknown error");
        try {
            window.parent.postMessage(
                { baseGpui: "demo-error", message: message.slice(0, 400) },
                "*",
            );
        } catch (_) {
            // A detached parent just means nobody is listening any more.
        }
    }
    window.addEventListener("error", function (event) {
        report(event.error || event.message);
    });
    window.addEventListener("unhandledrejection", function (event) {
        report(event.reason);
    });
})();
