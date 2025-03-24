<script setup lang='ts'>
import { useDataGridUpdater } from './hooks/useDataGridUpdater';
import DataGrid from './components/DataGrid.vue';
import SerialPanel from './components/SerialPanel.vue';
import RecordPanel from './components/RecordPanel.vue';

const { setupStateWatcher } = useDataGridUpdater();
setupStateWatcher();
</script>

<template>
  <div class='container' v-cloak>
    <SerialPanel />
    <RecordPanel />
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