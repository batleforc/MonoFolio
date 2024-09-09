<script lang="ts" setup>
import { useWasmStore } from '../stores/wasm';

// Import from the backend the pong wasm game


const wasmStore = useWasmStore();

const loadPong = async () => {
  if (!wasmStore.wasmLoading && wasmStore.moduleNames !== 'pong') {
    await wasmStore.fetchWasm('pong');
    wasmStore.runWasm();
  }
};
</script>

<template>
  <div class="min-w-full flex justify-center">
    <div>
      <Button v-if="wasmStore.moduleNames !== 'pong'" @click="loadPong" label="Load Pong" />
      <ProgressSpinner v-if="wasmStore.wasmLoading" />
    </div>
  </div>
  <canvas id="pong-bevy" width="800" height="600"></canvas>
</template>