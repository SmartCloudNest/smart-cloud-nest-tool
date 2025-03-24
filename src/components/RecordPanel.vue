<script setup lang="ts">
import { NText, NButtonGroup, NButton, NIcon, NDropdown } from 'naive-ui';
import { useRecordStore } from '../stores/record';
import { Pricetag } from '@vicons/ionicons5';

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
    <n-text class="tag-title">打标签</n-text>
    <n-button-group size="small">
      <n-button type="default" round>
        <template #icon>
          <n-icon class="tag-icon"><Pricetag /></n-icon>
        </template>
        左侧躺
      </n-button>
      <n-button type="default">
        <template #icon>
          <n-icon class="tag-icon"><Pricetag /></n-icon>
        </template>
        右侧躺
      </n-button>
      <n-button type="default">
        <template #icon>
          <n-icon class="tag-icon"><Pricetag /></n-icon>
        </template>
        平躺
      </n-button>
      <n-dropdown trigger="hover" :options="options" @select="">
        <n-button>更多</n-button>
      </n-dropdown>
    </n-button-group>
    <n-text v-if="recordStore.lastRecord"> 上一个标签: time: {{new Date(recordStore.lastRecord.time).toISOString()}}, tag: {{recordStore.lastRecord.tag}}</n-text>
  </div>
</template>

<style scoped>
.tag-title {
  color: goldenrod;
}

.tag-icon {
  color: darkslategray;
}

.record-panel {
  margin-top: 14px;
  display: flex;
  align-items: center;
  gap: 16px;
}

.n-button-group {
  display: flex;
}

.n-text {
  line-height: 1;
  display: flex;
  align-items: center;
}
</style>