<script setup>
  import {
  NGrid,
  NGridItem,
  NText,
} from 'naive-ui';
</script>

<template>
  <!-- 数据展示 -->
  <div v-if="store.dataSeq.length" style="margin-top: 24px">
    <n-text strong>实时数据序列：</n-text>
      <n-grid
        :cols="10"
        :x-gap="8"
        :y-gap="8"
        style="margin-top: 12px"
      >
      <n-grid-item
        v-for="(value, index) in reshapedData"
        :key="index"
      >
        <div class="data-item">
          <n-text type="info" depth="3">[{{ index }}]</n-text>
          <n-text strong>{{ value }}</n-text>
        </div>
      </n-grid-item>
    </n-grid>
  </div>
</template>

<style>
  .data-item {
    padding: 4px;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    display: flex;
    flex-direction: column;
    align-items: center;
    background-color: #fafafa;
  }
</style>