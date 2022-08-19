import { FlowComponent, JSX } from "solid-js"

export interface SlotsMap {
  [name: string]: Element[]
}

export class Slots {

  slots: SlotsMap

  constructor() {
    this.slots = {}
  }

  addSlot(name: string, el: Element) {
    if (!this.slots[name]) {
      this.slots[name] = []
    }
    this.slots[name].push(el)
  }

  getSlot(name: string): Element[] {
    return this.slots[name] || []
  }

  getDefaultSlot(): Element[] {
    return this.slots["default"] || []
  }

  setDefaultSlot(el: Element[]) {
    this.slots["default"] = el
  }
}

function cleanSlotElement(el: Element): Element {
  el.removeAttribute("slot")
  return el;
}

export function getSlotsFromElement(el: Element): Slots {
  const slots = new Slots()

  for (let i = 0; i < el.childNodes.length; i++) {
    if (el.childNodes[i].nodeName === '#text' && (el.childNodes[i] as Text).wholeText.trim() !== "") {
      slots.addSlot("default", el.childNodes[i] as Element)
    } else if ((el.childNodes[i] as Element).hasAttribute && !(el.childNodes[i] as Element).hasAttribute("slot")) {
      slots.addSlot("default", el.childNodes[i] as Element)
    }
  }

  el.querySelectorAll('[slot]').forEach(el => {
    const slotAttr = el.getAttribute('slot')
    if (slotAttr) {
      slots.addSlot(slotAttr.replace(
        /-(\w)/g,
        ($0, $1) => $1.toUpperCase())
      , cleanSlotElement(el));
    }
  });

  return slots
}

export type PropsMap = {
  [key: string]: any
}

export function getPropsFromElement(el: Element): PropsMap {
  let props: PropsMap = {}
  for (let i = 0; i < el.attributes.length; i++) {
    const attr = el.attributes[i]
    if (attr.name.includes("data-")) {
      props[attr.name.replace(/data-(\w)/g, ($0, $1) => $1)] = attr.value
    }
  }
  return props
}

export function getChildren(el: Element): Element[] {
  const contents: Element[] = []
  for(let i = 0; i <el.childNodes.length;i++) {
    contents.push(el.childNodes[i] as Element)
  }
  return contents
}

export function findElement<P>(elementName: string, cb: (el: Element, slots: Slots, props: P) => void): void {
  const els = document.getElementsByTagName(elementName)
  for(let i = 0; i < els.length; i++) {
    const slots = getSlotsFromElement(els[i])
    const props = getPropsFromElement(els[i])
    cb(els[i], slots, props as P)
  }
}