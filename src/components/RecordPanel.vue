<script setup lang='ts'>
import {
  NButton,
  NButtonGroup,
  NIcon,
  NText,
  NDropdown,
} from 'naive-ui';

import { Tag32Regular } from '@vicons/fluent';
import { useRecordStore } from '../stores/record';
import { computed } from 'vue';
import { saveDialog } from '../utils';
import { SerialState, usePortStore } from '../stores/port';

const recordStore = useRecordStore();
const portStore = usePortStore();

type Handler = () => void | Promise<void>;

async function handleAppendLayflat() {
  if (isPortDisabled) {
    return;
  }
  try {
    await recordStore.appendRecord("layflat", portStore.dataGrid);
  } catch (err) {
    throw err;
  }
}

async function handleAppendLeftside() {
  if (isPortDisabled) {
    return;
  }
  try {
    await recordStore.appendRecord("leftside", portStore.dataGrid);
  } catch (err) {
    throw err;
  }
}

async function handleAppendRightside() {
  if (isPortDisabled) {
    return;
  }
  try {
    await recordStore.appendRecord("rightside", portStore.dataGrid);
  } catch (err) {
    throw err;
  }
}

async function handleSave() {
  if (isRecordDisabled) {
    return;
  }
  try {
    const path = await saveDialog();
  await recordStore.saveRecords(path);
  } catch (err) {
    throw err;
  }
}

async function handlePop() {
  if (isRecordDisabled) {
    return;
  }
  try {
    await recordStore.popRecord();
  } catch (err) {
    throw err;
  }
}

async function handleReset() {
  if (isRecordDisabled) {
    return;
  }
  try {
    await recordStore.resetRecords();
  } catch (err) {
    throw err;
  }
}

const isRecordDisabled = computed(() => recordStore.recordLength === 0);
const isPortDisabled = computed(() => portStore.serialState !== SerialState.Connected);

type Keys = 'save' | 'pop' | 'reset';

const options = [
  {
    label: '保存',
    key: 'save',
    type: 'rander',
    disabled: isRecordDisabled.value,
  },
  {
    label: '*删除上一个记录',
    key: 'pop',
    type: 'rander',
    disabled: isRecordDisabled.value,
  },
  {
    label: '*删除全部记录',
    key: 'reset',
    type: 'rander',
    disabled: isRecordDisabled.value,
  },
];

async function handleSelect(key: Keys) {
  try {
    if (key === 'save') {
      await handleSave();
      return;
    }
    if (key === 'pop') {
      await handlePop();
      return;
    }
    if (key === 'reset') {
      await handleReset();
      return;
    }
  } catch (err) {
    throw err;
  }
}

</script>

<template>
  <div class='record-panel'>
    <n-text class='label-text' depth='3' strong>
       打标签
    </n-text>
    <n-button-group class='button-group' size='small'>
      <n-button :disabled="isPortDisabled" round>
        <template #icon>
          <n-icon><Tag32Regular /></n-icon>
        </template>
        左侧躺
      </n-button>
      <n-button :disabled="isPortDisabled" >
        <template #icon>
          <n-icon><Tag32Regular /></n-icon>
        </template>
        平躺
      </n-button>
      <n-button :disabled="isPortDisabled" >
        <template #icon>
          <n-icon><Tag32Regular /></n-icon>
        </template>
        右侧躺
      </n-button>
      <n-dropdown trigger="hover" :options="options" @select="handleSelect">
        <n-button>更多</n-button>
      </n-dropdown>
    </n-button-group>
    <n-text class='msg-text' depth='3' strong>
      {{ "123" }}
    </n-text>
  </div>
</template>

<style scoped>
.record-panel {
  display: flex;
  align-items: center;  /* 垂直居中 */
  gap: 12px;            /* 元素间距 */
  padding-top: 1%;
}

/* 文本基线对齐 */
.label-text {
  line-height: 3;      /* 根据实际字体大小调整 */
  margin-bottom: 1px;  /* 微调垂直偏移 */
  color: goldenrod;
}

/* 按钮组内部对齐一致 */
.button-group {
  display: inline-flex;
  align-items: center;
}

.msg-text {
  line-height: 1;      /* 根据实际字体大小调整 */
  margin-bottom: 1px;  /* 微调垂直偏移 */
  color:blueviolet;
}
</style>