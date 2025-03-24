<script setup lang="ts">
import { NText, NButtonGroup, NButton, NIcon, NDropdown } from 'naive-ui';
import { useRecordStore } from '../stores/record';

const recordStore = useRecordStore();

enum Keys {
  Save,
  Pop,
  Reset,
}

const options = [
  {
    label: '保存',
    key: Keys.Save,
    disabled: false,
  },
  {
    label: '*删除上一个',
    key: Keys.Pop,
    disabled: false,
  },
  {
    label: '*清空记录',
    key: Keys.Reset,
    disabled: false,
  },
];
</script>

<template>
  <div class='record-panel'> 
    <n-text>打标签</n-text>
    <n-button-group size="small">
      <n-button type="default" round>
        <template #icon>
          <n-icon><LogInIcon /></n-icon>
        </template>
        左侧躺
      </n-button>
      <n-button type="default">
        <template #icon>
          <n-icon><LogInIcon /></n-icon>
        </template>
        右侧躺
      </n-button>
      <n-button type="default">
        <template #icon>
          <n-icon><LogInIcon /></n-icon>
        </template>
        平躺
      </n-button>
      <n-dropdown trigger="hover" :options="options" @select="">
        <n-button>更多</n-button>
      </n-dropdown>
    </n-button-group>
    <n-text v-if="recordStore.lastRecord"> 标签添加成功！time: {{new Date(recordStore.lastRecord.time).toISOString()}}, tag: {{recordStore.lastRecord.tag}}</n-text>
  </div>
</template>

<style>
.record-panel {
  margin-top: 14px;
}
</style>