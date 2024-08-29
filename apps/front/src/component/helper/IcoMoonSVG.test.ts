// Test IcoMoonSVG component with vitest
import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import IcoMoonSVG from './IcoMoonSVG.vue';

describe('IcoMoonSVG', () => {
  it('renders properly', async () => {
    [
      ['home', 'text-red-500'],
      ['truc', 'machin'],
      ['chose', 'bidule'],
    ].forEach(([icon, className]) => {
      const wrapper = mount(IcoMoonSVG, {
        props: { icon, className },
      });
      expect(wrapper.attributes()['class']).toEqual(
        `ico ico-${icon} ${className}`,
      );
      expect(wrapper.attributes()['focusable']).toEqual('false');
      expect(wrapper.get('use').attributes()['href']).toEqual(
        `/icon/symbol-defs.svg#ico-${icon}`,
      );
    });
  });
});
