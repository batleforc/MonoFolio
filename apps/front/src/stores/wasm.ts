import { defineStore } from 'pinia';

export interface WasmState {
  wasmLoading: boolean;
  loadingError?: string;
  wasm?: unknown;
  moduleNames?: string;
}

const getUrl = (moduleName: string, extension: string) => {
  const baseUrl =
    import.meta.env.VITE_API_URL === ''
      ? window.location.origin
      : import.meta.env.VITE_API_URL;
  const url = new URL('/api/media', baseUrl);
  url.searchParams.append(
    'path',
    `${moduleName}/${moduleName}${extension === 'wasm' ? '_bg' : ''}.${extension}`,
  );
  return url.toString();
};

export const useWasmStore = defineStore({
  id: 'wasm',
  state: (): WasmState => ({
    wasmLoading: false,
  }),
  actions: {
    fetchWasm(moduleName: string) {
      if (this.wasmLoading) return new Promise<void>((resolve) => resolve());
      this.wasmLoading = true;
      this.moduleNames = moduleName;
      const jsModuleUrl = getUrl(moduleName, 'js');

      return import(/* @vite-ignore */ jsModuleUrl)
        .then((wasm) => {
          this.wasm = wasm;
        })
        .catch((err) => {
          console.error(err);
          this.loadingError = 'Failed to load wasm';
          this.moduleNames = '';
        })
        .finally(() => {
          this.wasmLoading = false;
        });
    },
    runWasm() {
      if (this.wasmLoading) return new Promise<void>((resolve) => resolve());
      this.wasmLoading = true;
      const wasmModuleUrl = getUrl(`${this.moduleNames}`, 'wasm');
      this.wasm.default({ module_or_path: wasmModuleUrl });
      this.wasmLoading = false;
    },
  },
});
