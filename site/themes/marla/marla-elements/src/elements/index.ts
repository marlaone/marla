import { FlowComponent, JSX } from "solid-js";
import { Slots } from "../utils";

export type MarlaProps<P> = {
  slots: Slots;
  props: P;
};

export type MarlaElement<P> = FlowComponent<
  MarlaProps<P>,
  Element[] | JSX.Element
>;

export * from "./hello-world";
