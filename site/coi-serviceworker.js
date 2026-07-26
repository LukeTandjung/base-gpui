// Cross-origin-isolation service worker for hosts that cannot set response
// headers (GitHub Pages). SharedArrayBuffer — required by GPUI's threaded
// wasm build — is only available in crossOriginIsolated contexts, which
// normally require the server to send COOP/COEP headers. A service worker is
// allowed to rewrite response headers, so this script injects them for every
// same-scope document and subresource. First-party reimplementation of the
// technique popularized by gzuidhof/coi-serviceworker.
//
// Include from every page that needs isolation (documents and iframe parents):
//   <script src="coi-serviceworker.js"></script>
// On first load it registers itself as a service worker and reloads the page
// once so the injected headers take effect.

if (typeof window === "undefined") {
    // Running as the service worker.
    self.addEventListener("install", () => self.skipWaiting());
    self.addEventListener("activate", (event) => event.waitUntil(self.clients.claim()));

    self.addEventListener("fetch", (event) => {
        const request = event.request;
        if (request.cache === "only-if-cached" && request.mode !== "same-origin") {
            return;
        }
        event.respondWith(
            fetch(request).then((response) => {
                if (response.status === 0) {
                    return response;
                }
                const headers = new Headers(response.headers);
                headers.set("Cross-Origin-Embedder-Policy", "require-corp");
                headers.set("Cross-Origin-Opener-Policy", "same-origin");
                return new Response(response.body, {
                    status: response.status,
                    statusText: response.statusText,
                    headers,
                });
            }),
        );
    });
} else if (!window.crossOriginIsolated && "serviceWorker" in navigator) {
    // Running in a page that is not yet isolated: register, then reload once
    // the worker takes control so the injected headers apply. clients.claim()
    // in the worker fires controllerchange here — listening for it (rather
    // than updatefound/statechange, which can fire before register() resolves)
    // is what makes the first-visit reload reliable.
    navigator.serviceWorker.addEventListener("controllerchange", () => {
        window.location.reload();
    });
    navigator.serviceWorker
        .register(document.currentScript.src)
        .then((registration) => {
            if (registration.active && !navigator.serviceWorker.controller) {
                // Registered on an earlier visit but this load is uncontrolled
                // (e.g. after a hard reload); reload to pick up the headers.
                window.location.reload();
            }
        })
        .catch((error) => console.error("coi-serviceworker registration failed:", error));
}
