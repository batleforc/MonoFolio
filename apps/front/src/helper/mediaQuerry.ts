import { computed, onMounted, onUnmounted, ref } from 'vue';

export const useBreakpoints = () => {
  const windowWidth = ref(window.innerWidth);

  const onWidthChange = () => (windowWidth.value = window.innerWidth);
  onMounted(() => window.addEventListener('resize', onWidthChange));
  onUnmounted(() => window.removeEventListener('resize', onWidthChange));

  const type = computed(() => {
    if (windowWidth.value < 680) return 'xs';
    if (windowWidth.value >= 680 && windowWidth.value < 1200) return 'md';
    if (windowWidth.value >= 1200) return 'lg';
    return null; // This is an unreachable line, simply to keep eslint happy.
  });

  const width = computed(() => windowWidth.value);

  return { width, type };
};
