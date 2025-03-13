<script setup lang='ts'>
import { ref } from 'vue'
import { save } from '@tauri-apps/plugin-dialog'
import { NText } from 'naive-ui'

const filePath = ref<string | null>(null)

async function handleClick() {
  try {
    const path = await save({
      title: '保存数据集文件',
      defaultPath: 'dataset.dat',
      filters: [{
        name: 'Dataset',
        extensions: ['dat', 'json']
      }]
    })
    
    if (path) {
      filePath.value = path
    }
  } catch (err) {
    console.error('保存失败:', err)
  }
}
</script>
