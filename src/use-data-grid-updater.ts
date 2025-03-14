import { ref, watch, onUnmounted } from 'vue';
import { usePortStore } from './stores/port';
import { sleep } from './utils';

export const useDataGridUpdater = () => {
  const portStore = usePortStore();
  const updateDataGridTask = ref<number | null>(null);

  const setupWatcher = () => {
    watch(() => portStore.serialState, async () => {
      if (portStore.isConnecting) return;
      
      if (portStore.isConnected) {
        updateDataGridTask.value = setInterval(portStore.updateDataGrid, updateDataGridTimeout);
        return;
      }

      if (portStore.isDisconnected && updateDataGridTask.value) {
        clearInterval(updateDataGridTask.value);
      }

      try {
        await portStore.connectPort();
      } catch (err) {
        await sleep(portReconnectTimeout);
        throw err;
      }
    });
  };

  onUnmounted(() => {
    if (updateDataGridTask.value) clearInterval(updateDataGridTask.value);
  });

  return { setupWatcher };
};
