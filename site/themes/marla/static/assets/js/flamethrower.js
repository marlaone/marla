function w(o) {
  ["link", "go"].includes(o) && window.scrollTo({ top: 0 });
}
function l(o) {
  const e = new URL(o || window.location.href).href;
  return e.endsWith("/") || e.includes(".") ? e : `${e}/`;
}
function m(o) {
  (!window.history.state || window.history.state.url !== o) && window.history.pushState({ url: o }, "internalLink", o);
}
function g(o) {
  document.querySelector(o).scrollIntoView({ behavior: "smooth", block: "start" });
}
function b(o) {
  const e = l();
  return { type: "popstate", next: e };
}
function y(o) {
  let e;
  if (o.altKey || o.ctrlKey || o.metaKey || o.shiftKey)
    return { type: "disqualified" };
  for (var t = o.target; t.parentNode; t = t.parentNode)
    if (t.nodeName === "A") {
      e = t;
      break;
    }
  if (e && e.host !== location.host)
    return e.target = "_blank", { type: "external" };
  if (e && "cold" in (e == null ? void 0 : e.dataset))
    return { type: "disqualified" };
  if (e != null && e.hasAttribute("href")) {
    const r = e.getAttribute("href"), n = new URL(r, location.href);
    if (o.preventDefault(), r != null && r.startsWith("#"))
      return g(r), { type: "scrolled" };
    const s = l(n.href), i = l();
    return { type: "link", next: s, prev: i };
  } else
    return { type: "noop" };
}
function v(o) {
  return new DOMParser().parseFromString(o, "text/html");
}
function u(o) {
  document.body.replaceWith(o.body);
}
function E(o) {
  const e = (i) => Array.from(i.querySelectorAll('head>:not([rel="prefetch"]')), t = e(document), r = e(o), { staleNodes: n, freshNodes: s } = k(t, r);
  n.forEach((i) => i.remove()), document.head.append(...s);
}
function k(o, e) {
  const t = [], r = [];
  let n = 0, s = 0;
  for (; n < o.length && s < e.length; ) {
    const i = o[n], c = e[s];
    if (i.isEqualNode(c)) {
      n++, s++;
      continue;
    }
    const h = r.findIndex((a) => a.isEqualNode(i));
    if (h !== -1) {
      r.splice(h, 1), n++;
      continue;
    }
    const d = t.findIndex((a) => a.isEqualNode(c));
    if (d !== -1) {
      t.splice(d, 1), s++;
      continue;
    }
    i && t.push(i), c && r.push(c), n++, s++;
  }
  return { staleNodes: t, freshNodes: r };
}
function f() {
  document.head.querySelectorAll("[data-reload]").forEach(p), document.body.querySelectorAll("script").forEach(p);
}
function p(o) {
  const e = document.createElement("script"), t = Array.from(o.attributes);
  for (const { name: r, value: n } of t)
    e[r] = n;
  e.append(o.textContent), o.replaceWith(e);
}
const F = {
  log: !1,
  pageTransitions: !1
};
class A {
  constructor(e) {
    this.opts = e, this.enabled = !0, this.prefetched = /* @__PURE__ */ new Set(), this.opts = { ...F, ...e != null ? e : {} }, window != null && window.history ? (document.addEventListener("click", (t) => this.onClick(t)), window.addEventListener("popstate", (t) => this.onPop(t)), this.prefetch()) : (console.warn(
      "flamethrower router not supported in this browser or environment"
    ), this.enabled = !1);
  }
  go(e) {
    const t = window.location.href, r = new URL(e, location.origin).href;
    return this.reconstructDOM({ type: "go", next: r, prev: t });
  }
  back() {
    window.history.back();
  }
  forward() {
    window.history.forward();
  }
  get allLinks() {
    return Array.from(document.links).filter(
      (e) => e.href.includes(document.location.origin) && !e.href.includes("#") && e.href !== (document.location.href || document.location.href + "/") && !this.prefetched.has(e.href)
    );
  }
  log(...e) {
    this.opts.log && console.log(...e);
  }
  prefetch() {
    if (this.opts.prefetch === "visible")
      this.prefetchVisible();
    else if (this.opts.prefetch === "hover")
      this.prefetchOnHover();
    else
      return;
  }
  prefetchOnHover() {
    this.allLinks.forEach((e) => {
      const t = e.getAttribute("href");
      e.addEventListener("pointerenter", () => this.createLink(t), { once: !0 });
    });
  }
  prefetchVisible() {
    const e = {
      root: null,
      rootMargin: "0px",
      threshold: 1
    };
    "IntersectionObserver" in window && (this.observer || (this.observer = new IntersectionObserver((t, r) => {
      t.forEach((n) => {
        const s = n.target.getAttribute("href");
        if (this.prefetched.has(s)) {
          r.unobserve(n.target);
          return;
        }
        n.isIntersecting && (this.createLink(s), r.unobserve(n.target));
      });
    }, e)), this.allLinks.forEach((t) => this.observer.observe(t)));
  }
  createLink(e) {
    const t = document.createElement("link");
    t.rel = "prefetch", t.href = e, t.as = "document", t.onload = () => this.log("\u{1F329}\uFE0F prefetched", e), t.onerror = (r) => this.log("\u{1F915} can't prefetch", e, r), document.head.appendChild(t), this.prefetched.add(e);
  }
  onClick(e) {
    this.reconstructDOM(y(e));
  }
  onPop(e) {
    this.reconstructDOM(b());
  }
  async reconstructDOM({ type: e, next: t, prev: r }) {
    if (!this.enabled) {
      this.log("router disabled");
      return;
    }
    try {
      if (this.log("\u26A1", e), ["popstate", "link", "go"].includes(e) && t !== r) {
        this.opts.log && console.time("\u23F1\uFE0F"), window.dispatchEvent(new CustomEvent("flamethrower:router:fetch")), m(t);
        const s = await (await fetch(t, { headers: { "X-Flamethrower": "1" } })).text(), i = v(s);
        E(i), this.opts.pageTransitions && document.createDocumentTransition ? document.createDocumentTransition().start(() => {
          u(i), f();
        }) : (u(i), f()), w(e), window.dispatchEvent(new CustomEvent("flamethrower:router:end")), setTimeout(() => {
          this.prefetch();
        }, 200), this.opts.log && console.timeEnd("\u23F1\uFE0F");
      }
    } catch (n) {
      return window.dispatchEvent(new CustomEvent("flamethrower:router:error", n)), this.opts.log && console.timeEnd("\u23F1\uFE0F"), console.error("\u{1F4A5} router fetch failed", n), !1;
    }
  }
}
const L = (o) => {
  const e = new A(o);
  if (o.log && console.log("\u{1F525} flamethrower engaged"), window) {
    const t = window;
    t.flamethrower = e;
  }
  return e;
};
export {
  L as default
};
