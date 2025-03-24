<script setup lang="ts">
import { NText, NButtonGroup, NButton, NIcon, NPopconfirm } from 'naive-ui';
import { useRecordStore } from '../stores/record';
import { Pricetag, Document } from '@vicons/ionicons5';
import { Important12Filled } from '@vicons/fluent';
import { saveDialog } from '../utils';
import { usePortStore } from '../stores/port';

const recordStore = useRecordStore();
const portStore = usePortStore();

async function handleSave() {
  try {
    const path = await saveDialog();
    recordStore.saveRecords(path);
  } catch (err) {
    throw err;
  }
}

async function handleLeftside() {
  try {
    await recordStore.appendRecord('leftside', portStore.dataGrid);
  } catch (err) {
    throw err;
  }
}

async function handleRightside() {
  try {
    await recordStore.appendRecord('rightside', portStore.dataGrid);
  } catch (err) {
    throw err;
  }
}

async function handleLayflat() {
  try {
    await recordStore.appendRecord('layflat', portStore.dataGrid);
  } catch (err) {
    throw err;
  }
}
</script>

<template>
  <div class='record-panel'> 
    <n-text class="tag-title">打标签</n-text>
    <n-button-group size="small">
      <n-button type="default" @click="handleLeftside" :disabled="!portStore.isConnected" round>
        <template #icon>
          <n-icon class="tag-icon"><Pricetag /></n-icon>
        </template>
        左侧躺
      </n-button>
      <n-button type="default" @click="handleRightside" :disabled="!portStore.isConnected">
        <template #icon>
          <n-icon class="tag-icon"><Pricetag /></n-icon>
        </template>
        右侧躺
      </n-button>
      <n-button type="default" @click="handleLayflat" :disabled="!portStore.isConnected">
        <template #icon>
          <n-icon class="tag-icon"><Pricetag /></n-icon>
        </template>
        平躺
      </n-button>
      <n-button type="default" @click="handleSave" :disabled="!recordStore.recordLength">
        <template #icon>
          <n-icon class="tag-icon"><Document /></n-icon>
        </template>
        保存
      </n-button>
      <n-popconfirm
        @positive-click="recordStore.popRecord"
        @negative-click=""
        positive-text="确认✅" negative-text="取消❌"
      >
        <template #trigger>
          <n-button :disabled="!recordStore.recordLength">
            <template #icon>
              <n-icon class="tag-icon"><Important12Filled /></n-icon>
            </template>
            删除上一个
          </n-button>
        </template>
        请确认！
      </n-popconfirm>
      <n-popconfirm
        @positive-click="recordStore.resetRecords"
        @negative-click=""
        positive-text="确认✅" negative-text="取消❌"
      >
        <template #trigger>
          <n-button :disabled="!recordStore.recordLength">
            <template #icon>
              <n-icon class="tag-icon"><Important12Filled /></n-icon>
            </template>
            清空记录
          </n-button>
        </template>
        此操作不可恢复，请谨慎操作！
      </n-popconfirm>
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