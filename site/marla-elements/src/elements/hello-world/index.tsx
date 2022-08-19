import { render } from 'solid-js/web';
import { findElement, getChildren } from '../../utils';
import type { MarlaElement } from '../'

type HelloWorldProps = { variant?: string }

const HelloWorld: MarlaElement<HelloWorldProps> = (ctx) => {
  return (
    <div class={`hello-world shadow-lg`}>
      <div>
        <div class="header">
          {ctx.slots.getSlot("header")}
        </div>
        <div class="default-content">
          {ctx.props.variant || "nope"}
          {ctx.slots.getDefaultSlot()}
        </div>
        <div class="footer">
          {ctx.slots.getSlot("footer")}
        </div>
      </div>
    </div>
  );
};

findElement<HelloWorldProps>('marla-hello-world', (el, slots, props) => {
  render(() => <HelloWorld slots={slots} props={props} children={[]} />, el as HTMLElement);
})