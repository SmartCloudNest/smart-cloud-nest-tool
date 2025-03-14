import { save } from '@tauri-apps/plugin-dialog';

export async function saveDialog(): Promise<string> {
    try {
        const path = await save({
            title: '创建新的数据集',
            defaultPath: 'jq-dataset.csv',
            filters: [{
                name: 'Dataset',
                extensions: ['csv'],
            }],
        });
        if (!path) {
            throw new Error('No path selected!');
        }
        return path;
    } catch (err) {
        throw err;
    }
}

export const sleep = (ms: number): Promise<void> => {
    return new Promise(resolve => setTimeout(resolve, ms));
}
  