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
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, watch } from 'vue';
import { useAppStore, SerialState } from './store'; // 导入 store
import {
  NSelect,
  NAlert,
  NTag,
  NGrid,
  NGridItem,
  NSpace,
  NText,
  NButton,
} from 'naive-ui';

const store = useAppStore();

// 串口选项
const portOptions = computed(() => {
  return store.serialports.map((port) => ({
    label: port,
    value: port,
  }));
});

// 是否已连接
const isConnected = computed(() => store.serialState === SerialState.Connected);

// 是否正在连接
const isConnecting = computed(() => store.serialState === SerialState.Connecting);

// 按钮文本
const buttonText = computed(() => {
  switch (store.serialState) {
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
  switch (store.serialState) {
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
  switch (store.serialState) {
    case SerialState.Connected:
      return `已连接: ${store.port}`;
    case SerialState.Connecting:
      return '正在连接...';
    default:
      return '未连接任何设备';
  }
});

// 处理连接/断开事件
const handleConnection = async () => {
  if (isConnected.value) {
    // 断开连接
    await store.resetSerialport();
  } else {
    // 连接串口
    if (store.selected_port) {
      // 直接修改 selected_port，watch 会自动触发连接逻辑
      store.selected_port = store.selected_port;
    } else {
      store.msg = '请先选择一个串口';
      // 三秒后清除提示信息
      setTimeout(() => {
        store.msg = '';
      }, 3000);
    }
  }
};

// 将数据重塑为 16x10 的表格
const reshapedData = computed(() => {
  const data = store.dataSeq;
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

// 定时更新串口列表
setInterval(() => {
  store.updateSerialports();
}, 500);

// 定时更新数据序列
setInterval(() => {
  if (store.serialState === SerialState.Connected) { // 检查连接状态
    store.updateDataSeq();
  }
}, 200);
</script>

<style scoped>
.container {
  padding: 24px;
  max-width: 800px;
  margin: 0 auto;
}

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
