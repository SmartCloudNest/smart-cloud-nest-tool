<script setup lang='ts'>
import { computed, onUnmounted, ref, watch } from 'vue';
import {
  NSelect,
  NTag,
  NSpace,
  NButton,
  NButtonGroup,
  NDropdown,
  NText,
} from 'naive-ui';
import { usePortStore, SerialState } from './stores/port';
import {
  commandAppendRecord,
  commandSaveRecords,
  commandPopRecord,
  commandResetRecords,
  commandRecordsLen,
  commandLastRecord,
} from './commands';
import DataGrid from './components/DataGrid.vue';
import { save } from '@tauri-apps/plugin-dialog';

const portStore = usePortStore();

async function handleClick() {
  try {
    const path = await save({
      title: '创建新的数据集',
      defaultPath: 'jq-dataset.csv',
      filters: [{
        name: 'Dataset',
        extensions: ['csv'],
      }]
    });

    // todo
    
    if (!path) {
      return;
    } 
  } catch (err) {
    console.error('保存失败:', err);
  }
}

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

const sleep = (ms: number): Promise<void> => {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const updateDataGridTask = ref<number | null>(null);

const recordCount = ref(0);
const lastRecord = ref<CsvRecord | null>(null);

async function updateRecordInfo() {
  recordCount.value = await commandRecordsLen();
  lastRecord.value = await commandLastRecord();
}

async function handleAppend(tag: string) {
  await commandAppendRecord(tag, '');
  await updateRecordInfo();
}

async function handleSave() {
  try {
    const path = await save({
      title: '保存记录',
      defaultPath: 'jq-records.csv',
      filters: [{
        name: 'CSV',
        extensions: ['csv'],
      }]
    });
    if (path) {
      await commandSaveRecords(path);
    }
  } catch (err) {
    console.error('保存失败:', err);
  }
}

async function handlePop() {
  await commandPopRecord();
  await updateRecordInfo();
}

async function handleReset() {
  await commandResetRecords();
  await updateRecordInfo();
}

const dropdownOptions = [
  {
    label: '保存',
    key: 'save',
  },
  {
    label: '删除最后数据',
    key: 'pop',
  },
  {
    label: '重置记录',
    key: 'reset',
  }
];

function handleDropdownSelect(key: string) {
  switch (key) {
    case 'save':
      handleSave();
      break;
    case 'pop':
      handlePop();
      break;
    case 'reset':
      handleReset();
      break;
  }
}

watch(() => portStore.serialState, async () => {
  if (portStore.isConnecting) {
    return;
  }
  if (portStore.isConnected) {
    updateDataGridTask.value = setInterval(portStore.updateDataGrid, 72);
    return;
  }
  if (portStore.isDisconnected && updateDataGridTask.value) {
    clearInterval(updateDataGridTask.value);
  }
  try {
    await portStore.connectPort();
  } catch (err) {
    await sleep(330);
    throw err;
  }
})

onUnmounted(() => {
  if (updateDataGridTask.value) {
    clearInterval(updateDataGridTask.value);
  }
})

</script>
<template>
  <div class='container' v-cloak>
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
    <n-space vertical>
      <n-space align='center' :size='8'>
        <n-button-group>
          <n-button @click='handleAppend("layflat")'>平躺</n-button>
          <n-button @click='handleAppend("leftside")'>左侧</n-button>
          <n-button @click='handleAppend("rightside")'>右侧</n-button>
        </n-button-group>
        <n-dropdown
          trigger='click'
          :options='dropdownOptions'
          @select='handleDropdownSelect'
        >
          <n-button>操作</n-button>
        </n-dropdown>
        <n-text>
          当前记录{{ recordCount }}条，
          上次记录时间{{ lastRecord?.time ? new Date(lastRecord.time).toLocaleString() : '无' }}，
          标签为{{ lastRecord?.tag || '无' }}
        </n-text>
      </n-space>
    </n-space>
    <DataGrid :data-grid='portStore.dataGrid' />
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