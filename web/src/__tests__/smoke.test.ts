import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import { createPinia, setActivePinia, defineStore } from 'pinia';

describe('Test Infrastructure', () => {
  it('renders a basic component', () => {
    const TestComp = { template: '<div>Hello Vitest</div>' };
    const wrapper = mount(TestComp);
    expect(wrapper.text()).toBe('Hello Vitest');
  });

  it('creates a Pinia store', () => {
    const pinia = createPinia();
    setActivePinia(pinia);
    const useTest = defineStore('test', () => ({ value: 'works' }));
    const store = useTest();
    expect(store.value).toBe('works');
  });
});
