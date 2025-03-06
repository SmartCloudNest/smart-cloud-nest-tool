<script setup lang="ts">
import { onMounted, watch } from 'vue';
import { useAppStore } from './store';

const store = useAppStore();

// 组件挂载时立即获取可用串口列表
onMounted(() => {
  store.updateSerialports();
});

// 监听选择的串口，当成功连接时开始自动获取数据
watch(
  () => store.getPort,
  (newPort) => {
    if (newPort) {
      // 连接成功后立即获取数据
      store.setSerialport(newPort);
      // 设置定时轮询数据（每秒一次）
      const timer = setInterval(() => {
        if (store.getPort) {  // 确保仍处于连接状态
          store.updateDataSeq();
        } else {
          clearInterval(timer);
        }
      }, 1000);
    }
  }
);
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
      <button @click="store.updateSerialports">刷新列表</button>
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

<style scoped>
.container {
  max-width: 800px;
  margin: 20px auto;
  padding: 20px;
}

.error-message {
  color: #dc3545;
  background: #f8d7da;
  padding: 10px;
  border-radius: 4px;
  margin-bottom: 20px;
}

.control-group {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
  align-items: center;
}

select {
  padding: 5px 10px;
  min-width: 200px;
}

button {
  padding: 5px 10px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover {
  background: #0056b3;
}

.status {
  margin: 15px 0;
  font-weight: bold;
}

.disconnected {
  color: #6c757d;
}

.data-container {
  margin-top: 20px;
}

.data-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 10px;
}

.data-item {
  background: #f8f9fa;
  padding: 10px;
  border-radius: 4px;
  text-align: center;
}

.index {
  color: #6c757d;
  font-size: 0.8em;
}

.value {
  display: block;
  font-size: 1.2em;
  font-weight: bold;
}
</style>
