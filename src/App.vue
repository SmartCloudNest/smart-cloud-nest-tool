<script setup lang='ts'>
import { NSpace } from 'naive-ui';
import PortSelect from './components/PortSelect.vue';
import DataGrid from './components/DataGrid.vue';
import { onUnmounted, ref, watch } from 'vue';
import { usePortStore } from './stores/port';
import { sleep } from './utils';

const portStore = usePortStore();
const updateDataGridTask = ref<number | null>(null);

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

onUnmounted(() => {
  if (updateDataGridTask.value) clearInterval(updateDataGridTask.value);
});

</script>

<template>
  <div class='container' v-cloak>
    <n-space vertical>
      <PortSelect />
      <DataGrid />
    </n-space>
  </div>
</template>

<style>
.container {
  overflow: hidden;
  padding: 24px;
  max-width: 800px;
  margin: 0 auto;
}
</style>
