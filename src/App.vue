<script setup lang='ts'>
import { onUnmounted, watch } from 'vue';
import { usePortStore } from './stores/port';
import { useDataGridUpdater } from './hooks/useDataGridUpdater';
import DataGrid from './components/DataGrid.vue';
import SerialPanel from './components/SerialPanel.vue';
import { sleep } from './utils';

const { setupStateWatcher } = useDataGridUpdater();

setupStateWatcher();

</script>
<template>
  <div class='container' v-cloak>
    <SerialPanel />
    <DataGrid />
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