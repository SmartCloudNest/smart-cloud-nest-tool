<script setup lang="ts">
import { computed, onMounted, watch } from 'vue';
import { usePortStore, SerialState } from './stores/port';
import {
  NSelect,
  NAlert,
  NTag,
  NSpace,
  NText,
  NButton,
} from 'naive-ui';

const portStore = usePortStore();

// 串口选项
const portOptions = computed(() => {
  return portStore.serialports.map((port) => ({
    label: port,
    value: port,
  }));
});

// 是否已连接
const isConnected = computed(() => portStore.serialState === SerialState.Connected);

// 是否正在连接
const isConnecting = computed(() => portStore.serialState === SerialState.Connecting);

// 按钮文本
const buttonText = computed(() => {
  switch (portStore.serialState) {
    case SerialState.Connected:
      return '断开';
    case SerialState.Connecting:
      return '正在连接...';
    default:
      return '连接';
  }
});

// 状态标签类型
const statusTagType = computed(() => {
  switch (portStore.serialState) {
    case SerialState.Connected:
      return 'success';
    case SerialState.Connecting:
      return 'warning';
    default:
      return 'error';
  }
});

// 状态文本
const statusText = computed(() => {
  switch (portStore.serialState) {
    case SerialState.Connected:
      return `已连接: ${portStore.port}`;
    case SerialState.Connecting:
      return '正在连接...';
    default:
      return '未连接任何设备';
  }
});

// 将数据重塑为 16x10 的表格
const reshapedData = computed(() => {
  const data = portStore.dataSeq;
  const reshaped = [];
  for (let i = 0; i < 16; i++) {
    for (let j = 0; j < 10; j++) {
      const index = i * 10 + j;
      reshaped.push(data[index] || '-');
    }
  }
  return reshaped;
});

// 组件挂载时获取串口列表
onMounted(() => {
  store.updateSerialports();
});
</script>

<template>
  <div class="container">
    <!-- 错误信息展示 -->
    <n-alert
      v-if="store.msg"
      :title="store.msg"
      type="error"
      :bordered="false"
      style="margin-bottom: 16px"
    >
      {{ store.msg }}
    </n-alert>

    <!-- 串口选择部分 -->
    <n-space vertical>
      <n-space align="center" :size="8">
        <n-text strong>选择串口：</n-text>
        <n-select
          v-model:value="store.selected_port"
          :options="portOptions"
          :disabled="!store.serialports.length"
          placeholder="请选择串口"
          style="width: 200px"
        />
        <!-- 连接/断开按钮 -->
        <n-button
          :type="isConnected ? 'error' : 'primary'"
          :disabled="isConnecting || !store.selected_port"
          :loading="isConnecting"
          @click="handleConnection"
        >
          {{ buttonText }}
        </n-button>
      </n-space>

      <!-- 连接状态显示 -->
      <n-tag :type="statusTagType">
        {{ statusText }}
      </n-tag>
    </n-space>
  </div>
</template>

<style>
.container {
  padding: 24px;
  max-width: 800px;
  margin: 0 auto;
}
</style>