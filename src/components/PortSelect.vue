<script setup lang='ts'>
import { NSelect, NTag, NSpace, NText } from 'naive-ui';
import { usePortStore } from '../stores/port';
import { computed } from 'vue';

const portStore = usePortStore();

const handleUpdate = (value: string) => {
  portStore.port = value;
  portStore.connectPort();
};

const portOptions = computed(() => 
  portStore.serialports.map(port => ({ label: port, value: port }))
);

</script>

<template>
  <n-space align='center' :size='6'>
    <n-tag :type='portStore.tagType'>
      {{ portStore.tagText }}
    </n-tag>
    <n-text strong>选择串口：</n-text>
    <n-select
      :value='portStore.port'
      @update:show='portStore.updateSerialports'
      @update:value='handleUpdate'
      :options='portOptions'
      placeholder='请选择串口'
      style='width: 200px'
    />
  </n-space>
</template>
