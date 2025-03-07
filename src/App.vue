<script setup lang="ts">
import { onMounted } from 'vue';
import { useAppStore } from './store';

const store = useAppStore();

// 组件挂载时立即获取可用串口列表
onMounted(() => {
  store.updateSerialports();
});

setInterval(() => {
  store.updateSerialports();
}, 500);

setInterval(() => {
  if (store.getPort) {
    store.updateDataSeq();
  }
}, 150);

</script>

<template>
  <div class="container">
    <!-- 错误信息展示 -->
    <div v-if="store.getMsg" class="error-message">
      {{ store.getMsg }}
    </div>

    <!-- 串口选择部分 -->
    <div class="control-group">
      <label for="serial-port-select">选择串口：</label>
      <select 
        id="serial-port-select" 
        :disabled="!store.getSerialports"
        @change="(e) => store.setSerialport((e.target as HTMLSelectElement).value)"
      >
        <option value=undefined>请选择串口</option>
        <option 
          v-for="port in store.getSerialports" 
          :key="port" 
          :value="port"
          :selected="store.getPort === port"
        >
          {{ port }}
        </option>
      </select>
    </div>

    <!-- 连接状态显示 -->
    <div class="status">
      <span v-if="store.getPort">
        已连接: {{ store.getPort }}
      </span>
      <span v-else class="disconnected">
        未连接任何设备
      </span>
    </div>

    <!-- 数据展示 -->
    <div class="data-container" v-if="store.getDataSeq?.length">
      <h3>实时数据序列：</h3>
      <div class="data-grid">
        <div 
          v-for="(value, index) in store.getDataSeq" 
          :key="index" 
          class="data-item"
        >
          <span class="index">[{{ index }}]</span>
          <span class="value">{{ value }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
