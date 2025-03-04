<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 串口相关状态
const selectedPort = ref("");
const serialPorts = ref<string[]>([]);
const serialData = ref<number[]>([]);
const errorMsg = ref("");

// 初始化时获取可用串口列表
onMounted(async () => {
  try {
    serialPorts.value = await invoke("command_get_serialports");
  } catch (err) {
    errorMsg.value = `获取串口失败: ${err}`;
  }
});

// 设置选中的串口
async function setSerialPort() {
  if (!selectedPort.value) return;

  try {
    await invoke("command_set_serialport", { name: selectedPort.value });
    errorMsg.value = "";
  } catch (err) {
    errorMsg.value = `连接串口失败: ${err}`;
  }
}

// 获取串口数据
async function getSerialData() {
  try {
    const data = await invoke<number[]>("command_get_serial_data");
    serialData.value = data;
    errorMsg.value = "";
  } catch (err) {
    errorMsg.value = `读取数据失败: ${err}`;
  }
}
</script>

<template>
  <main class="container">
    <!-- 串口选择区域 -->
    <div class="row">
      <select v-model="selectedPort" class="custom-select">
        <option disabled value="">请选择串口</option>
        <option 
          v-for="port in serialPorts" 
          :key="port" 
          :value="port"
        >
          {{ port }}
        </option>
      </select>
      
      <button 
        @click="setSerialPort" 
        :disabled="!selectedPort"
        class="action-button"
      >
        连接串口
      </button>
    </div>

    <!-- 数据操作区域 -->
    <div class="row">
      <button 
        @click="getSerialData" 
        :disabled="!selectedPort"
        class="data-button"
      >
        获取数据
      </button>
    </div>

    <!-- 数据显示 -->
    <div v-if="serialData.length" class="data-container">
      <h3>接收数据 (十六进制):</h3>
      <pre>{{ serialData.map(b => b.toString(16).padStart(2, '0')).join(' ') }}</pre>
    </div>

    <!-- 错误信息 -->
    <div v-if="errorMsg" class="error-message">
      {{ errorMsg }}
    </div>
  </main>
</template>
