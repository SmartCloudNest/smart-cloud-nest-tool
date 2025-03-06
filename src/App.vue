<template>
  <div class="container">
    <!-- 串口操作区域 -->
    <div class="control-group">
      <button @click="store.updateSerialports">刷新串口列表</button>
      
      <select 
        v-model="selectedPort"
        @change="handlePortChange"
        :disabled="!store.serialports?.length"
      >
        <option value="">请选择串口</option>
        <option 
          v-for="port in store.serialports" 
          :key="port" 
          :value="port"
        >
          {{ port }}
        </option>
      </select>
      
      <button 
        @click="store.updateDataSeq"
        :disabled="!store.getPort"
        class="primary"
      >
        获取数据
      </button>
    </div>

    <!-- 状态显示 -->
    <div class="status-group">
      <p>当前连接：{{ store.getPort || '未连接' }}</p>
      <div v-if="store.getMsg" class="error">
        {{ store.getMsg }}
      </div>
    </div>

    <!-- 数据展示 -->
    <div v-if="store.getDataSeq" class="data-container">
      <h3>数据序列：</h3>
      <div class="data-grid">
        <div 
          v-for="(value, index) in store.getDataSeq" 
          :key="index"
          class="data-item"
        >
          {{ value }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useAppStore } from './store'

const store = useAppStore()
const selectedPort = ref<string>('')

// 自动初始化串口列表
onMounted(() => {
  store.updateSerialports()
})

// 处理端口选择变化
const handlePortChange = async () => {
  if (selectedPort.value) {
    await store.setSerialport(selectedPort.value)
    selectedPort.value = '' // 清空选择器值
  }
}
</script>

<style scoped>
.container {
  max-width: 800px;
  margin: 2rem auto;
  padding: 1rem;
}

.control-group {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
}

select {
  flex: 1;
  padding: 0.5rem;
}

button {
  padding: 0.5rem 1rem;
  cursor: pointer;
  background: #f0f0f0;
  border: 1px solid #ccc;
}

button.primary {
  background: #007bff;
  color: white;
  border-color: #007bff;
}

.status-group {
  margin-bottom: 2rem;
}

.error {
  color: #dc3545;
  margin-top: 0.5rem;
}

.data-container {
  background: #f8f9fa;
  padding: 1rem;
  border-radius: 4px;
}

.data-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(60px, 1fr));
  gap: 0.5rem;
  margin-top: 1rem;
}

.data-item {
  padding: 0.5rem;
  background: white;
  border: 1px solid #dee2e6;
  text-align: center;
  border-radius: 4px;
}
</style>
