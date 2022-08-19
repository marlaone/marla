(function(b){typeof define=="function"&&define.amd?define(b):b()})(function(){"use strict";const b="",S={};let W=R;const E={},y=1,A=2,P={owned:null,cleanups:null,context:null,owner:null};var h=null;let N=null,a=null,m=null,u=null,p=null,T=0;function Q(t,s){const e=a,l=h,n=t.length===0,o=n?P:{owned:null,cleanups:null,context:null,owner:s||l},f=n?t:()=>t(()=>D(o));h=o,a=null;try{return B(f,!0)}finally{a=e,h=l}}function v(t,s,e){const l=J(t,s,!1,y);I(l)}function V(t){if(m)return t();let s;const e=m=[];try{s=t()}finally{m=null}return B(()=>{for(let l=0;l<e.length;l+=1){const n=e[l];if(n.pending!==E){const o=n.pending;n.pending=E,L(n,o)}}},!1),s}function F(t){let s,e=a;return a=null,s=t(),a=e,s}function L(t,s,e){if(m)return t.pending===E&&m.push(t),t.pending=s,s;if(t.comparator&&t.comparator(t.value,s))return s;let l=!1;return t.value=s,t.observers&&t.observers.length&&B(()=>{for(let n=0;n<t.observers.length;n+=1){const o=t.observers[n];l&&N.disposed.has(o),(l&&!o.tState||!l&&!o.state)&&(o.pure?u.push(o):p.push(o),o.observers&&q(o)),l||(o.state=y)}if(u.length>1e6)throw u=[],new Error},!1),s}function I(t){if(!t.fn)return;D(t);const s=h,e=a,l=T;a=h=t,j(t,t.value,l),a=e,h=s}function j(t,s,e){let l;try{l=t.fn(s)}catch(n){G(n)}(!t.updatedAt||t.updatedAt<=e)&&(t.observers&&t.observers.length?L(t,l):t.value=l,t.updatedAt=e)}function J(t,s,e,l=y,n){const o={fn:t,state:l,updatedAt:null,owned:null,sources:null,sourceSlots:null,cleanups:null,value:s,owner:h,context:null,pure:e};return h===null||h!==P&&(h.owned?h.owned.push(o):h.owned=[o]),o}function O(t){const s=N;if(t.state===0||s)return;if(t.state===A||s)return $(t);if(t.suspense&&F(t.suspense.inFallback))return t.suspense.effects.push(t);const e=[t];for(;(t=t.owner)&&(!t.updatedAt||t.updatedAt<T);)(t.state||s)&&e.push(t);for(let l=e.length-1;l>=0;l--)if(t=e[l],t.state===y||s)I(t);else if(t.state===A||s){const n=u;u=null,$(t,e[0]),u=n}}function B(t,s){if(u)return t();let e=!1;s||(u=[]),p?e=!0:p=[],T++;try{const l=t();return K(e),l}catch(l){u||(p=null),G(l)}}function K(t){u&&(R(u),u=null),!t&&(p.length?V(()=>{W(p),p=null}):p=null)}function R(t){for(let s=0;s<t.length;s++)O(t[s])}function $(t,s){const e=N;t.state=0;for(let l=0;l<t.sources.length;l+=1){const n=t.sources[l];n.sources&&(n.state===y||e?n!==s&&O(n):(n.state===A||e)&&$(n,s))}}function q(t){const s=N;for(let e=0;e<t.observers.length;e+=1){const l=t.observers[e];(!l.state||s)&&(l.state=A,l.pure?u.push(l):p.push(l),l.observers&&q(l))}}function D(t){let s;if(t.sources)for(;t.sources.length;){const e=t.sources.pop(),l=t.sourceSlots.pop(),n=e.observers;if(n&&n.length){const o=n.pop(),f=e.observerSlots.pop();l<n.length&&(o.sourceSlots[f]=l,n[l]=o,e.observerSlots[l]=f)}}if(t.owned){for(s=0;s<t.owned.length;s++)D(t.owned[s]);t.owned=null}if(t.cleanups){for(s=0;s<t.cleanups.length;s++)t.cleanups[s]();t.cleanups=null}t.state=0,t.context=null}function G(t){throw t}function X(t,s){return F(()=>t(s||{}))}function Y(t,s,e){let l=e.length,n=s.length,o=l,f=0,i=0,c=s[n-1].nextSibling,r=null;for(;f<n||i<o;){if(s[f]===e[i]){f++,i++;continue}for(;s[n-1]===e[o-1];)n--,o--;if(n===f){const d=o<l?i?e[i-1].nextSibling:e[o-i]:c;for(;i<o;)t.insertBefore(e[i++],d)}else if(o===i)for(;f<n;)(!r||!r.has(s[f]))&&s[f].remove(),f++;else if(s[f]===e[o-1]&&e[i]===s[n-1]){const d=s[--n].nextSibling;t.insertBefore(e[i++],s[f++].nextSibling),t.insertBefore(e[--o],d),s[n]=e[o]}else{if(!r){r=new Map;let g=i;for(;g<o;)r.set(e[g],g++)}const d=r.get(s[f]);if(d!=null)if(i<d&&d<o){let g=f,_=1,M;for(;++g<n&&g<o&&!((M=r.get(s[g]))==null||M!==d+_);)_++;if(_>d-i){const ot=s[f];for(;i<d;)t.insertBefore(e[i++],ot)}else t.replaceChild(e[i++],s[f++])}else f++;else s[f++].remove()}}}function Z(t,s,e){let l;return Q(n=>{l=n,s===document?t():x(s,t(),s.firstChild?null:void 0,e)}),()=>{l(),s.textContent=""}}function z(t,s,e){const l=document.createElement("template");l.innerHTML=t;let n=l.content.firstChild;return e&&(n=n.firstChild),n}function x(t,s,e,l){if(e!==void 0&&!l&&(l=[]),typeof s!="function")return C(t,s,l,e);v(n=>C(t,s(),n,e),l)}function C(t,s,e,l,n){for(S.context&&!e&&(e=[...t.childNodes]);typeof e=="function";)e=e();if(s===e)return e;const o=typeof s,f=l!==void 0;if(t=f&&e[0]&&e[0].parentNode||t,o==="string"||o==="number"){if(S.context)return e;if(o==="number"&&(s=s.toString()),f){let i=e[0];i&&i.nodeType===3?i.data=s:i=document.createTextNode(s),e=w(t,e,l,i)}else e!==""&&typeof e=="string"?e=t.firstChild.data=s:e=t.textContent=s}else if(s==null||o==="boolean"){if(S.context)return e;e=w(t,e,l)}else{if(o==="function")return v(()=>{let i=s();for(;typeof i=="function";)i=i();e=C(t,i,e,l)}),()=>e;if(Array.isArray(s)){const i=[],c=e&&Array.isArray(e);if(U(i,s,e,n))return v(()=>e=C(t,i,e,l,!0)),()=>e;if(S.context){for(let r=0;r<i.length;r++)if(i[r].parentNode)return e=i}if(i.length===0){if(e=w(t,e,l),f)return e}else c?e.length===0?H(t,i,l):Y(t,e,i):(e&&w(t),H(t,i));e=i}else if(s instanceof Node){if(S.context&&s.parentNode)return e=f?[s]:s;if(Array.isArray(e)){if(f)return e=w(t,e,l,s);w(t,e,null,s)}else e==null||e===""||!t.firstChild?t.appendChild(s):t.replaceChild(s,t.firstChild);e=s}}return e}function U(t,s,e,l){let n=!1;for(let o=0,f=s.length;o<f;o++){let i=s[o],c=e&&e[o];if(i instanceof Node)t.push(i);else if(!(i==null||i===!0||i===!1))if(Array.isArray(i))n=U(t,i,c)||n;else if(typeof i=="function")if(l){for(;typeof i=="function";)i=i();n=U(t,Array.isArray(i)?i:[i],c)||n}else t.push(i),n=!0;else{const r=String(i);c&&c.nodeType===3&&c.data===r?t.push(c):t.push(document.createTextNode(r))}}return n}function H(t,s,e){for(let l=0,n=s.length;l<n;l++)t.insertBefore(s[l],e)}function w(t,s,e,l){if(e===void 0)return t.textContent="";const n=l||document.createTextNode("");if(s.length){let o=!1;for(let f=s.length-1;f>=0;f--){const i=s[f];if(n!==i){const c=i.parentNode===t;!o&&!f?c?t.replaceChild(n,i):t.insertBefore(n,e):c&&i.remove()}else o=!0}}else t.insertBefore(n,e);return[n]}class k{constructor(){this.slots={}}addSlot(s,e){this.slots[s]||(this.slots[s]=[]),this.slots[s].push(e)}getSlot(s){return this.slots[s]||[]}getDefaultSlot(){return this.slots.default||[]}setDefaultSlot(s){this.slots.default=s}}function tt(t){return t.removeAttribute("slot"),t}function et(t){const s=new k;for(let e=0;e<t.childNodes.length;e++)(t.childNodes[e].nodeName==="#text"&&t.childNodes[e].wholeText.trim()!==""||t.childNodes[e].hasAttribute&&!t.childNodes[e].hasAttribute("slot"))&&s.addSlot("default",t.childNodes[e]);return t.querySelectorAll("[slot]").forEach(e=>{const l=e.getAttribute("slot");l&&s.addSlot(l.replace(/-(\w)/g,(n,o)=>o.toUpperCase()),tt(e))}),s}function st(t){let s={};for(let e=0;e<t.attributes.length;e++){const l=t.attributes[e];l.name.includes("data-")&&(s[l.name.replace(/data-(\w)/g,(n,o)=>o)]=l.value)}return s}function lt(t,s){const e=document.getElementsByTagName(t);for(let l=0;l<e.length;l++){const n=et(e[l]),o=st(e[l]);s(e[l],n,o)}}const nt=z('<div class="hello-world shadow-lg"><div><div class="header"></div><div class="default-content"></div><div class="footer"></div></div></div>'),it=t=>(()=>{const s=nt.cloneNode(!0),e=s.firstChild,l=e.firstChild,n=l.nextSibling,o=n.nextSibling;return x(l,()=>t.slots.getSlot("header")),x(n,()=>t.props.variant||"nope",null),x(n,()=>t.slots.getDefaultSlot(),null),x(o,()=>t.slots.getSlot("footer")),s})();lt("marla-hello-world",(t,s,e)=>{Z(()=>X(it,{slots:s,props:e,children:[]}),t)})});
