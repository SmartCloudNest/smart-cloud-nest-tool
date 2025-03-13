<script setup lang='ts'>
import {
  NGrid,
  NGridItem,
  NText
} from 'naive-ui';

interface Props {
  dataGrid: number[][];
}

defineProps<Props>();
</script>

<template>
  <!-- 数据展示 -->
  <div style='margin-top: 24px'>
    <n-grid :cols='14' :x-gap='8' :y-gap='8' style='margin-top: 24px'>
      <template v-for='(row, rowIndex) in dataGrid'>
        <n-grid-item v-for='(value, colIndex) in row'>
          <div class='data-item'>
            <n-text type='info' depth='3'>[{{ rowIndex }},{{ colIndex }}]</n-text>
            <n-text strong>{{ value || '-' }}</n-text>
          </div>
        </n-grid-item>
      </template>
    </n-grid>
  </div>
</template>

<style>
.data-item {
  padding: 1px;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  display: flex;
  flex-direction: column;
  align-items: center;
  background-color: #fafafa;
}
</style>
