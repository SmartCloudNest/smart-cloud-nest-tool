import { save } from "@tauri-apps/plugin-dialog";

export async function saveDialog(): Promise<string> {
    try {
      const path = await save({
        title: '创建新的数据集',
        defaultPath: 'jq-dataset.csv',
        filters: [{
          name: 'Dataset',
          extensions: ['csv'],
        }]
      });
      if (!path) {
        throw new Error('未指定path')
      } 
      return path;
    } catch (err) {
      throw err;
    }
}

export function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
  