const S = {};
let W = q;
const E = {}, A = 1, m = 2, F = {
  owned: null,
  cleanups: null,
  context: null,
  owner: null
};
var h = null;
let C = null, p = null, x = null, c = null, d = null, $ = 0;
function Q(t, s) {
  const e = p, l = h, n = t.length === 0, o = n ? F : {
    owned: null,
    cleanups: null,
    context: null,
    owner: s || l
  }, f = n ? t : () => t(() => U(o));
  h = o, p = null;
  try {
    return D(f, !0);
  } finally {
    p = e, h = l;
  }
}
function T(t, s, e) {
  const l = J(t, s, !1, A);
  O(l);
}
function V(t) {
  if (x)
    return t();
  let s;
  const e = x = [];
  try {
    s = t();
  } finally {
    x = null;
  }
  return D(() => {
    for (let l = 0; l < e.length; l += 1) {
      const n = e[l];
      if (n.pending !== E) {
        const o = n.pending;
        n.pending = E, I(n, o);
      }
    }
  }, !1), s;
}
function L(t) {
  let s, e = p;
  return p = null, s = t(), p = e, s;
}
function I(t, s, e) {
  if (x)
    return t.pending === E && x.push(t), t.pending = s, s;
  if (t.comparator && t.comparator(t.value, s))
    return s;
  let l = !1;
  return t.value = s, t.observers && t.observers.length && D(() => {
    for (let n = 0; n < t.observers.length; n += 1) {
      const o = t.observers[n];
      l && C.disposed.has(o), (l && !o.tState || !l && !o.state) && (o.pure ? c.push(o) : d.push(o), o.observers && G(o)), l || (o.state = A);
    }
    if (c.length > 1e6)
      throw c = [], new Error();
  }, !1), s;
}
function O(t) {
  if (!t.fn)
    return;
  U(t);
  const s = h, e = p, l = $;
  p = h = t, j(t, t.value, l), p = e, h = s;
}
function j(t, s, e) {
  let l;
  try {
    l = t.fn(s);
  } catch (n) {
    H(n);
  }
  (!t.updatedAt || t.updatedAt <= e) && (t.observers && t.observers.length ? I(t, l) : t.value = l, t.updatedAt = e);
}
function J(t, s, e, l = A, n) {
  const o = {
    fn: t,
    state: l,
    updatedAt: null,
    owned: null,
    sources: null,
    sourceSlots: null,
    cleanups: null,
    value: s,
    owner: h,
    context: null,
    pure: e
  };
  return h === null || h !== F && (h.owned ? h.owned.push(o) : h.owned = [o]), o;
}
function R(t) {
  const s = C;
  if (t.state === 0 || s)
    return;
  if (t.state === m || s)
    return v(t);
  if (t.suspense && L(t.suspense.inFallback))
    return t.suspense.effects.push(t);
  const e = [t];
  for (; (t = t.owner) && (!t.updatedAt || t.updatedAt < $); )
    (t.state || s) && e.push(t);
  for (let l = e.length - 1; l >= 0; l--)
    if (t = e[l], t.state === A || s)
      O(t);
    else if (t.state === m || s) {
      const n = c;
      c = null, v(t, e[0]), c = n;
    }
}
function D(t, s) {
  if (c)
    return t();
  let e = !1;
  s || (c = []), d ? e = !0 : d = [], $++;
  try {
    const l = t();
    return K(e), l;
  } catch (l) {
    c || (d = null), H(l);
  }
}
function K(t) {
  c && (q(c), c = null), !t && (d.length ? V(() => {
    W(d), d = null;
  }) : d = null);
}
function q(t) {
  for (let s = 0; s < t.length; s++)
    R(t[s]);
}
function v(t, s) {
  const e = C;
  t.state = 0;
  for (let l = 0; l < t.sources.length; l += 1) {
    const n = t.sources[l];
    n.sources && (n.state === A || e ? n !== s && R(n) : (n.state === m || e) && v(n, s));
  }
}
function G(t) {
  const s = C;
  for (let e = 0; e < t.observers.length; e += 1) {
    const l = t.observers[e];
    (!l.state || s) && (l.state = m, l.pure ? c.push(l) : d.push(l), l.observers && G(l));
  }
}
function U(t) {
  let s;
  if (t.sources)
    for (; t.sources.length; ) {
      const e = t.sources.pop(), l = t.sourceSlots.pop(), n = e.observers;
      if (n && n.length) {
        const o = n.pop(), f = e.observerSlots.pop();
        l < n.length && (o.sourceSlots[f] = l, n[l] = o, e.observerSlots[l] = f);
      }
    }
  if (t.owned) {
    for (s = 0; s < t.owned.length; s++)
      U(t.owned[s]);
    t.owned = null;
  }
  if (t.cleanups) {
    for (s = 0; s < t.cleanups.length; s++)
      t.cleanups[s]();
    t.cleanups = null;
  }
  t.state = 0, t.context = null;
}
function H(t) {
  throw t;
}
function X(t, s) {
  return L(() => t(s || {}));
}
function Y(t, s, e) {
  let l = e.length, n = s.length, o = l, f = 0, i = 0, u = s[n - 1].nextSibling, r = null;
  for (; f < n || i < o; ) {
    if (s[f] === e[i]) {
      f++, i++;
      continue;
    }
    for (; s[n - 1] === e[o - 1]; )
      n--, o--;
    if (n === f) {
      const a = o < l ? i ? e[i - 1].nextSibling : e[o - i] : u;
      for (; i < o; )
        t.insertBefore(e[i++], a);
    } else if (o === i)
      for (; f < n; )
        (!r || !r.has(s[f])) && s[f].remove(), f++;
    else if (s[f] === e[o - 1] && e[i] === s[n - 1]) {
      const a = s[--n].nextSibling;
      t.insertBefore(e[i++], s[f++].nextSibling), t.insertBefore(e[--o], a), s[n] = e[o];
    } else {
      if (!r) {
        r = /* @__PURE__ */ new Map();
        let g = i;
        for (; g < o; )
          r.set(e[g], g++);
      }
      const a = r.get(s[f]);
      if (a != null)
        if (i < a && a < o) {
          let g = f, b = 1, _;
          for (; ++g < n && g < o && !((_ = r.get(s[g])) == null || _ !== a + b); )
            b++;
          if (b > a - i) {
            const M = s[f];
            for (; i < a; )
              t.insertBefore(e[i++], M);
          } else
            t.replaceChild(e[i++], s[f++]);
        } else
          f++;
      else
        s[f++].remove();
    }
  }
}
function Z(t, s, e) {
  let l;
  return Q((n) => {
    l = n, s === document ? t() : y(s, t(), s.firstChild ? null : void 0, e);
  }), () => {
    l(), s.textContent = "";
  };
}
function z(t, s, e) {
  const l = document.createElement("template");
  l.innerHTML = t;
  let n = l.content.firstChild;
  return e && (n = n.firstChild), n;
}
function y(t, s, e, l) {
  if (e !== void 0 && !l && (l = []), typeof s != "function")
    return N(t, s, l, e);
  T((n) => N(t, s(), n, e), l);
}
function N(t, s, e, l, n) {
  for (S.context && !e && (e = [...t.childNodes]); typeof e == "function"; )
    e = e();
  if (s === e)
    return e;
  const o = typeof s, f = l !== void 0;
  if (t = f && e[0] && e[0].parentNode || t, o === "string" || o === "number") {
    if (S.context)
      return e;
    if (o === "number" && (s = s.toString()), f) {
      let i = e[0];
      i && i.nodeType === 3 ? i.data = s : i = document.createTextNode(s), e = w(t, e, l, i);
    } else
      e !== "" && typeof e == "string" ? e = t.firstChild.data = s : e = t.textContent = s;
  } else if (s == null || o === "boolean") {
    if (S.context)
      return e;
    e = w(t, e, l);
  } else {
    if (o === "function")
      return T(() => {
        let i = s();
        for (; typeof i == "function"; )
          i = i();
        e = N(t, i, e, l);
      }), () => e;
    if (Array.isArray(s)) {
      const i = [], u = e && Array.isArray(e);
      if (B(i, s, e, n))
        return T(() => e = N(t, i, e, l, !0)), () => e;
      if (S.context) {
        for (let r = 0; r < i.length; r++)
          if (i[r].parentNode)
            return e = i;
      }
      if (i.length === 0) {
        if (e = w(t, e, l), f)
          return e;
      } else
        u ? e.length === 0 ? P(t, i, l) : Y(t, e, i) : (e && w(t), P(t, i));
      e = i;
    } else if (s instanceof Node) {
      if (S.context && s.parentNode)
        return e = f ? [s] : s;
      if (Array.isArray(e)) {
        if (f)
          return e = w(t, e, l, s);
        w(t, e, null, s);
      } else
        e == null || e === "" || !t.firstChild ? t.appendChild(s) : t.replaceChild(s, t.firstChild);
      e = s;
    }
  }
  return e;
}
function B(t, s, e, l) {
  let n = !1;
  for (let o = 0, f = s.length; o < f; o++) {
    let i = s[o], u = e && e[o];
    if (i instanceof Node)
      t.push(i);
    else if (!(i == null || i === !0 || i === !1))
      if (Array.isArray(i))
        n = B(t, i, u) || n;
      else if (typeof i == "function")
        if (l) {
          for (; typeof i == "function"; )
            i = i();
          n = B(t, Array.isArray(i) ? i : [i], u) || n;
        } else
          t.push(i), n = !0;
      else {
        const r = String(i);
        u && u.nodeType === 3 && u.data === r ? t.push(u) : t.push(document.createTextNode(r));
      }
  }
  return n;
}
function P(t, s, e) {
  for (let l = 0, n = s.length; l < n; l++)
    t.insertBefore(s[l], e);
}
function w(t, s, e, l) {
  if (e === void 0)
    return t.textContent = "";
  const n = l || document.createTextNode("");
  if (s.length) {
    let o = !1;
    for (let f = s.length - 1; f >= 0; f--) {
      const i = s[f];
      if (n !== i) {
        const u = i.parentNode === t;
        !o && !f ? u ? t.replaceChild(n, i) : t.insertBefore(n, e) : u && i.remove();
      } else
        o = !0;
    }
  } else
    t.insertBefore(n, e);
  return [n];
}
class k {
  constructor() {
    this.slots = {};
  }
  addSlot(s, e) {
    this.slots[s] || (this.slots[s] = []), this.slots[s].push(e);
  }
  getSlot(s) {
    return this.slots[s] || [];
  }
  getDefaultSlot() {
    return this.slots.default || [];
  }
  setDefaultSlot(s) {
    this.slots.default = s;
  }
}
function tt(t) {
  return t.removeAttribute("slot"), t;
}
function et(t) {
  const s = new k();
  for (let e = 0; e < t.childNodes.length; e++)
    (t.childNodes[e].nodeName === "#text" && t.childNodes[e].wholeText.trim() !== "" || t.childNodes[e].hasAttribute && !t.childNodes[e].hasAttribute("slot")) && s.addSlot("default", t.childNodes[e]);
  return t.querySelectorAll("[slot]").forEach((e) => {
    const l = e.getAttribute("slot");
    l && s.addSlot(
      l.replace(
        /-(\w)/g,
        (n, o) => o.toUpperCase()
      ),
      tt(e)
    );
  }), s;
}
function st(t) {
  let s = {};
  for (let e = 0; e < t.attributes.length; e++) {
    const l = t.attributes[e];
    l.name.includes("data-") && (s[l.name.replace(/data-(\w)/g, (n, o) => o)] = l.value);
  }
  return s;
}
function lt(t, s) {
  const e = document.getElementsByTagName(t);
  for (let l = 0; l < e.length; l++) {
    const n = et(e[l]), o = st(e[l]);
    s(e[l], n, o);
  }
}
const nt = /* @__PURE__ */ z('<div class="hello-world shadow-lg"><div><div class="header"></div><div class="default-content"></div><div class="footer"></div></div></div>'), it = (t) => (() => {
  const s = nt.cloneNode(!0), e = s.firstChild, l = e.firstChild, n = l.nextSibling, o = n.nextSibling;
  return y(l, () => t.slots.getSlot("header")), y(n, () => t.props.variant || "nope", null), y(n, () => t.slots.getDefaultSlot(), null), y(o, () => t.slots.getSlot("footer")), s;
})();
lt("marla-hello-world", (t, s, e) => {
  Z(() => X(it, {
    slots: s,
    props: e,
    children: []
  }), t);
});
