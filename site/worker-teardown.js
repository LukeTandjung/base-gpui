// The threaded wasm build spawns worker threads that each hold the compiled
// module alive. On reload the browser compiles a fresh copy while the old
// workers still pin the previous one, and the overlap can exhaust the
// executable-memory budget. Track every worker and terminate them the instant
// the document goes away so the old instance is reclaimable before the new
// compile starts.
//
// Must load (synchronously, in <head>) before the trunk-injected wasm glue so
// the Worker wrapper is in place when the thread pool spawns.
(function () {
    var NativeWorker = window.Worker;
    var workers = [];
    window.Worker = function (url, opts) {
        var worker = new NativeWorker(url, opts);
        workers.push(worker);
        return worker;
    };
    window.Worker.prototype = NativeWorker.prototype;
    window.addEventListener("pagehide", function () {
        for (var i = 0; i < workers.length; i++) {
            try {
                workers[i].terminate();
            } catch (_) {}
        }
        workers.length = 0;
    });
})();
