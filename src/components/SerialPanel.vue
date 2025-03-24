<script setup lang='ts'>
import { computed, onUnmounted, ref, watch } from 'vue';
import { NSelect, NTag, NSpace, NText } from 'naive-ui';
import { usePortStore, SerialState } from '../stores/port';
import { sleep } from '../utils';

const portStore = usePortStore();

const portOptions = computed(() => {
  return portStore.serialports.map((port) => ({
    label: port,
    value: port,
  }));
});

const tagText = computed(() => {
  switch (portStore.serialState) {
    case SerialState.Connected:
      return `已连接: ${portStore.port}`;
    case SerialState.Connecting:
      return `正在连接: ${portStore.port}`;
    default:
      return '未连接任何设备';
  }
});

const tagType = computed(() => {
  switch (portStore.serialState) {
    case SerialState.Connected:
      return 'success';
    case SerialState.Connecting:
      return 'info';
    default:
      return 'warning';
  }
});

// const updateDataGridTask = ref<number | null>(null);

// watch(() => portStore.serialState, async () => {
//   if (portStore.isConnecting) {
//     return;
//   }
//   if (portStore.isConnected) {
//     updateDataGridTask.value = setInterval(portStore.updateDataGrid, 72);
//     return;
//   }
//   if (portStore.isDisconnected && updateDataGridTask.value) {
//     clearInterval(updateDataGridTask.value);
//   }
//   try {
//     await portStore.connectPort();
//   } catch (err) {
//     await sleep(330);
//     throw err;
//   }
// });

// onUnmounted(() => {
//   if (updateDataGridTask.value) {
//     clearInterval(updateDataGridTask.value);
//   }
// });
</script>

<template>
  <n-space vertical>
    <n-space align='center' :size='8'>
      <n-tag :type='tagType'>
        {{ tagText }}
      </n-tag>
      <n-text strong>选择串口：</n-text>
      <n-select
        v-model:value='portStore.port'
        v-on:update:show='portStore.updateSerialports'
        v-on:update:value='portStore.connectPort'
        :options='portOptions'
        placeholder='请选择串口'  
        style='width: 200px'
      />
    </n-space>
  </n-space>
</template>