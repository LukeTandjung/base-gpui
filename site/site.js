// Docs site client script. Page-specific data comes from the document: the
// deployment base path from <html data-base>, and the search index from the
// inline window.__SEARCH_INDEX__ assignment each page carries.
(function () {
  var base = document.documentElement.getAttribute("data-base") || "";
  var index = window.__SEARCH_INDEX__ || [];

  // ── Examples: hide the skeleton once the iframe paints. Swap to the
  // fallback when WebGPU is missing or yields no adapter (`navigator.gpu`
  // often exists on mobile while requestAdapter() resolves null), and to the
  // error state when the demo shell reports a boot failure via postMessage.
  var demoCards = Array.prototype.slice.call(document.querySelectorAll(".demo"));
  // Phones can't run the demos (WebGPU adapter and shared-memory limits), so
  // say so up front and skip the ~18 MB wasm download entirely. Tablets pass
  // through and get a real attempt — iPads report a desktop UA anyway.
  var isPhone = navigator.userAgentData
    ? navigator.userAgentData.mobile
    : /iPhone|iPod|Android.*Mobile/i.test(navigator.userAgent);
  if (isPhone) {
    demoCards.forEach(function (demo) {
      var frame = demo.querySelector("iframe");
      if (frame && frame.getAttribute("src")) {
        frame.dataset.src = frame.src;
        frame.removeAttribute("src");
      }
      demo.dataset.state = "mobile";
      var tryButton = demo.querySelector("[data-demo-try]");
      if (tryButton) tryButton.addEventListener("click", function () {
        demo.dataset.state = "loading";
        if (frame && frame.dataset.src) frame.src = frame.dataset.src;
      });
    });
  }
  demoCards.forEach(function (demo) {
    var frame = demo.querySelector("iframe");
    if (frame) frame.addEventListener("load", function () {
      if (demo.dataset.state === "loading") demo.dataset.state = "ready";
    });
  });
  if (demoCards.length) {
    if (!isPhone && !("gpu" in navigator)) {
      demoCards.forEach(function (demo) { demo.dataset.state = "unsupported"; });
    } else if (!isPhone) {
      navigator.gpu.requestAdapter().then(function (adapter) {
        if (adapter) return;
        demoCards.forEach(function (demo) { demo.dataset.state = "unsupported"; });
      }, function () {});
    }
    window.addEventListener("message", function (event) {
      var data = event.data;
      if (!data || data.baseGpui !== "demo-error") return;
      demoCards.forEach(function (demo) {
        var frame = demo.querySelector("iframe");
        if (!frame || frame.contentWindow !== event.source) return;
        demo.dataset.state = "failed";
        var detail = demo.querySelector("[data-demo-error]");
        if (detail) detail.textContent = String(data.message || "unknown error");
      });
    });
  }

  // ── Each demo iframe holds a compiled wasm module, worker threads, and a
  // WebGPU device. Pages restored later via the back/forward cache would keep
  // all of that resident, and a few navigations exhaust the browser's
  // executable-memory budget ("failed to allocate executable memory"). Unload
  // the iframe when the page is hidden and restore it if the page comes back.
  window.addEventListener("pagehide", function () {
    document.querySelectorAll(".demo iframe").forEach(function (frame) {
      if (!frame.src) return;
      frame.dataset.src = frame.src;
      frame.removeAttribute("src");
      if (frame.contentWindow) frame.contentWindow.location.replace("about:blank");
      var demo = frame.closest(".demo");
      if (demo) demo.dataset.state = "loading";
    });
  });
  window.addEventListener("pageshow", function (event) {
    if (!event.persisted) return;
    document.querySelectorAll(".demo iframe").forEach(function (frame) {
      if (frame.dataset.src) frame.src = frame.dataset.src;
    });
  });

  // ── On this page: highlight the section in view.
  var tocLinks = Array.prototype.slice.call(document.querySelectorAll(".toc__link"));
  if (tocLinks.length && "IntersectionObserver" in window) {
    var byId = {};
    tocLinks.forEach(function (link) { byId[link.getAttribute("href").slice(1)] = link; });
    var visible = [];
    var observer = new IntersectionObserver(function (entries) {
      entries.forEach(function (entry) {
        var id = entry.target.id;
        var at = visible.indexOf(id);
        if (entry.isIntersecting && at === -1) visible.push(id);
        if (!entry.isIntersecting && at !== -1) visible.splice(at, 1);
      });
      var order = Object.keys(byId);
      var active = order.filter(function (id) { return visible.indexOf(id) !== -1; })[0];
      tocLinks.forEach(function (link) { link.classList.remove("is-active"); });
      if (active && byId[active]) byId[active].classList.add("is-active");
    }, { rootMargin: "0px 0px -70% 0px" });
    Object.keys(byId).forEach(function (id) {
      var target = document.getElementById(id);
      if (target) observer.observe(target);
    });
  }

  // ── Search dialog.
  var palette = document.getElementById("palette");
  var input = document.getElementById("palette-input");
  var results = document.getElementById("palette-results");
  if (!palette || !input || !results) return;
  var active = 0;
  var rows = [];

  function score(entry, query) {
    var title = entry.t.toLowerCase();
    if (title === query) return 0;
    if (title.indexOf(query) === 0) return 1;
    if (title.indexOf(query) !== -1) return 2;
    for (var i = 0; i < entry.s.length; i += 1) {
      if (entry.s[i].n.toLowerCase().indexOf(query) !== -1) return 3;
    }
    return -1;
  }

  function render(query) {
    var q = query.trim().toLowerCase();
    var items = [];
    index.forEach(function (entry) {
      if (!q) { items.push({ title: entry.t, sub: entry.g, url: entry.u, rank: 1 }); return; }
      var rank = score(entry, q);
      if (rank === -1) return;
      items.push({ title: entry.t, sub: entry.g, url: entry.u, rank: rank });
      entry.s.forEach(function (section) {
        if (section.n.toLowerCase().indexOf(q) === -1) return;
        items.push({ title: section.n, sub: entry.t, url: entry.u + "#" + section.a, rank: rank + 0.5 });
      });
    });
    items.sort(function (a, b) { return a.rank - b.rank; });
    items = items.slice(0, 40);
    rows = items;
    active = 0;
    if (!items.length) {
      results.innerHTML = '<li class="palette__empty">No matches</li>';
      return;
    }
    results.innerHTML = items
      .map(function (item, i) {
        return '<li class="palette__result' + (i === 0 ? " is-active" : "") + '"><a href="' + base + item.url + '"><strong>' + item.title + "</strong><span>" + item.sub + "</span></a></li>";
      })
      .join("");
  }

  function setActive(next) {
    var nodes = results.querySelectorAll(".palette__result");
    if (!nodes.length) return;
    active = (next + nodes.length) % nodes.length;
    Array.prototype.forEach.call(nodes, function (node, i) { node.classList.toggle("is-active", i === active); });
    nodes[active].querySelector("a").focus({ preventScroll: true });
    input.focus({ preventScroll: true });
  }

  function open() {
    palette.setAttribute("open", "");
    input.value = "";
    render("");
    input.focus();
  }
  function close() { palette.removeAttribute("open"); }

  document.querySelectorAll("[data-search-open]").forEach(function (button) {
    button.addEventListener("click", open);
  });
  document.querySelectorAll("[data-search-close]").forEach(function (node) {
    node.addEventListener("click", close);
  });
  input.addEventListener("input", function () { render(input.value); });
  document.addEventListener("keydown", function (event) {
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
      event.preventDefault();
      palette.hasAttribute("open") ? close() : open();
      return;
    }
    if (!palette.hasAttribute("open")) return;
    if (event.key === "Escape") { event.preventDefault(); close(); }
    if (event.key === "ArrowDown") { event.preventDefault(); setActive(active + 1); }
    if (event.key === "ArrowUp") { event.preventDefault(); setActive(active - 1); }
    if (event.key === "Enter") {
      var current = rows[active];
      if (current) { event.preventDefault(); window.location.href = base + current.url; }
    }
  });
})();
