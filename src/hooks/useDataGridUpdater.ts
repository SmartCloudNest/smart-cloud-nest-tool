import { Ref, ref, watch, onUnmounted } from 'vue';
import { usePortStore } from '../stores/port';
import { sleep } from '../utils';

export function useDataGridUpdater() {
  const portStore = usePortStore();
  const timerRef: Ref<number | null> = ref(null);

  const setupDataGridUpdater = () => {
    timerRef.value = setInterval(portStore.updateDataGrid, 72);
  };

  const cleanupDataGridUpdater = () => {
    if (timerRef.value) {
      clearInterval(timerRef.value);
      timerRef.value = null;
    }
  };

  const setupStateWatcher = () => {
    watch(() => portStore.serialState, async () => {
      if (portStore.isConnecting) return;
      
      if (portStore.isConnected) {
        setupDataGridUpdater();
        return;
      }

      if (portStore.isDisconnected) {
        cleanupDataGridUpdater();
      }

      try {
        await portStore.connectPort();
      } catch (err) {
        await sleep(330);
        throw err;
      }
    });
  };

  onUnmounted(cleanupDataGridUpdater);

  return { setupStateWatcher };
}