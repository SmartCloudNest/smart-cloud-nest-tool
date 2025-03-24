<script setup lang='ts'>
import { useDataGridUpdater } from './hooks/useDataGridUpdater';
import DataGrid from './components/DataGrid.vue';
import SerialPanel from './components/SerialPanel.vue';
import { NText, NButtonGroup, NButton, NIcon, NDropdown } from 'naive-ui';

const { setupStateWatcher } = useDataGridUpdater();

setupStateWatcher();

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
  <div class='container' v-cloak>
    <SerialPanel />
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
          <n-button>悬浮！</n-button>
        </n-dropdown>
      </n-button-group>
    </div>
    
    <DataGrid />
  </div>
</template>

<style>
.record-panel {
  margin-top: 14px;
}
</style>

<style>
.container {
  overflow: hidden;
  padding: 24px;
  max-width: 800px;
  margin: 0 auto;
}
</style>