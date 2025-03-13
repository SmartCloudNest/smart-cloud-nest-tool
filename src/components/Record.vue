<template>
    <n-space vertical>
      <!-- 状态展示卡片 -->
      <n-card title="当前记录状态" segmented>
        <n-space vertical>
          <n-statistic label="记录总数" :value="recordLength" />
          <n-space>
            <n-tag v-if="lastRecord" type="success">
              最后记录时间：{{ formatDate(lastRecord.timestamp) }}
            </n-tag>
            <n-tag v-else type="default">暂无记录</n-tag>
            
            <n-tag v-if="lastDeleted" type="error">
              最后删除时间：{{ formatDate(lastDeleted.timestamp) }}
            </n-tag>
          </n-space>
        </n-space>
      </n-card>
  
      <!-- 操作按钮组 -->
      <n-card title="记录操作">
        <n-space>
          <n-button @click="handleAdd" type="primary">
            <template #icon>
              <n-icon><add-icon /></n-icon>
            </template>
            添加随机记录
          </n-button>
  
          <n-button @click="handlePop" type="warning" :disabled="recordLength === 0">
            <template #icon>
              <n-icon><remove-icon /></n-icon>
            </template>
            删除最后记录
          </n-button>
  
          <n-divider vertical />
  
          <n-button @click="showSaveDialog = true" type="info">
            <template #icon>
              <n-icon><save-icon /></n-icon>
            </template>
            保存记录
          </n-button>
  
          <n-button @click="showResetConfirm = true" type="error">
            <template #icon>
              <n-icon><trash-icon /></n-icon>
            </template>
            清除所有记录
          </n-button>
        </n-space>
      </n-card>
  
      <!-- 记录详情表格 -->
      <n-card v-if="lastRecord" title="最近记录详情">
        <n-data-table
          :columns="columns"
          :data="[lastRecord]"
          :bordered="false"
          max-height="300px"
        />
      </n-card>
  
      <!-- 保存对话框 -->
      <n-modal v-model:show="showSaveDialog">
        <n-card
          title="保存记录"
          style="width: 600px"
          :bordered="false"
          size="huge"
        >
          <n-input
            v-model:value="savePath"
            placeholder="请输入保存路径"
            @keydown.enter="handleSave"
          />
          <template #footer>
            <n-space justify="end">
              <n-button @click="showSaveDialog = false">取消</n-button>
              <n-button type="primary" @click="handleSave">确认保存</n-button>
            </n-space>
          </template>
        </n-card>
      </n-modal>
  
      <!-- 重置确认对话框 -->
      <n-modal v-model:show="showResetConfirm">
        <n-card
          title="确认清除所有记录"
          style="width: 500px"
          :bordered="false"
          size="huge"
        >
          <n-text depth="3">这将永久删除所有记录，不可恢复！</n-text>
          <template #footer>
            <n-space justify="end">
              <n-button @click="showResetConfirm = false">取消</n-button>
              <n-button type="error" @click="handleReset">确认清除</n-button>
            </n-space>
          </template>
        </n-card>
      </n-modal>
    </n-space>
  </template>
  
  <script setup lang="ts">
  import { ref } from 'vue'
  import { usePortStore } from '../stores/port'
  import {
    NButton,
    NTag,
    NCard,
    NSpace,
    NStatistic,
    NDataTable,
    NModal,
    NInput,
    NDivider,
    NText,
    useNotification
  } from 'naive-ui'
  import {
    SaveOutline as SaveIcon,
    TrashOutline as TrashIcon,
    AddCircleOutline as AddIcon,
    RemoveCircleOutline as RemoveIcon
  } from '@vicons/ionicons5'
import { CsvRecord } from '../commands'
  
  const store = usePortStore()
  const notification = useNotification()
  
  // 保存对话框状态
  const showSaveDialog = ref(false)
  const savePath = ref('')
  // 重置确认对话框
  const showResetConfirm = ref(false)
  
  // 表格列配置
  const columns = [
    {
      title: '时间',
      key: 'timestamp',
      render: (row: CsvRecord) => formatDate(row.time)
    },
    { title: '标签', key: 'tag' },
    {
      title: '数据',
      key: 'data',
      render: (row: CsvRecord) => JSON.stringify(JSON.parse(row.data))
    }
  ]
  
  // 日期格式化方法
  const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleString()
  }
  
  // 添加示例记录
  const handleAdd = async () => {
    try {
      const mockData = Array.from({ length: 3 }, () =>
        Array.from({ length: 3 }, () => Math.random() * 100)
      )
      await store.appendRecord(`记录_${Date.now()}`, mockData)
      notification.success({
        content: '添加成功',
        duration: 2000
      })
    } catch (error) {
      notification.error({
        content: `添加失败: ${error}`,
        duration: 3000
      })
    }
  }
  
  // 删除最后记录
  const handlePop = async () => {
    try {
      await store.popRecord()
      notification.success({
        content: '已删除最后一条记录',
        duration: 2000
      })
    } catch (error) {
      notification.error({
        content: `删除失败: ${error}`,
        duration: 3000
      })
    }
  }
  
  // 保存记录
  const handleSave = async () => {
    try {
      if (!savePath.value) {
        notification.warning({
          content: '请输入保存路径',
          duration: 2000
        })
        return
      }
      await store.saveRecords(savePath.value)
      showSaveDialog.value = false
      notification.success({
        content: '保存成功',
        duration: 2000
      })
    } catch (error) {
      notification.error({
        content: `保存失败: ${error}`,
        duration: 3000
      })
    }
  }
  
  // 重置记录
  const handleReset = async () => {
    try {
      await store.resetRecords()
      showResetConfirm.value = false
      notification.success({
        content: '已清除所有记录',
        duration: 2000
      })
    } catch (error) {
      notification.error({
        content: `清除失败: ${error}`,
        duration: 3000
      })
    }
  }
  </script>
  
  <style scoped>
  .n-card {
    margin-bottom: 16px;
  }
  </style>
  