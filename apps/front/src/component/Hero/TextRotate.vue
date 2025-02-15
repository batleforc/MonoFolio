<script setup lang="ts">
import { ref, onMounted } from 'vue';

const props = defineProps<{
  texts: Array<string>;
}>();

const textsInDiv = ref("");
const timeoutId = ref<number | null>(0);

const tick = (textIncomming: string, deleting: boolean, loopNum: number) => {
  let fullText = props.texts[loopNum % props.texts.length];
  let futurText = fullText.substring(0, textIncomming.length + (deleting ? -1 : 1));
  textsInDiv.value = futurText;
  let delta = 150 - Math.random() * 100;
  if (deleting) { delta /= 2; }
  if (!deleting && futurText === fullText) {
    delta = 1000;
    deleting = true;
  } else if (deleting && futurText === '') {
    deleting = false;
    loopNum++;
    delta = 400;
  }
  var id = setTimeout(() => tick(futurText, deleting, loopNum), delta);
  timeoutId.value = id;
}

onMounted(() => {
  tick(textsInDiv.value, false, 0);
  return () => {
    if (timeoutId.value) {
      clearTimeout(timeoutId.value);
    }
  };
});
</script>

<template>
  <div class="txt-rotate-wrapper">
    <span class="txt-rotate">
      <span class="wrap">{{ textsInDiv }}</span>
    </span>
  </div>
</template>

<style scoped>
.txt-rotate-wrapper {
  display: flex;
}

.txt-rotate>.wrap {
  border-right: 0.08em solid #666;
}
</style>