import { ref, watch } from 'vue';
import { commandGetSerialports, commandSetSerialport, commandGetDataSeq, commandResetSerialport } from './commands';
import { defineStore } from 'pinia';

export enum SerialState {
    Connected,
    Connecting,
    Disconnected,
}

export const useAppStore = defineStore('app', () => {
    const port = ref<string>('');
    const dataSeq = ref<number[]>([]);
    const msg = ref<string>('');
    const serialports = ref<string[]>([]);
    const selected_port = ref<string>('');
    const serialState = ref<SerialState>(SerialState.Disconnected);

    const updateSerialports = async () => {
        try {
            serialports.value = await commandGetSerialports();
        } catch (err) {
            msg.value = `获取串口列表失败： ${err}`;
        }
    };

    const setSerialport = async (name: string) => {
        try {
            serialState.value = SerialState.Connecting; // 设置为正在连接
            await commandSetSerialport(name);
            port.value = name; // 更新连接的端口
            serialState.value = SerialState.Connected; // 设置为已连接
        } catch (err) {
            msg.value = `无法连接到串口： ${err}`;
            serialState.value = SerialState.Disconnected; // 连接失败，设置为未连接
        }
    };

    const updateDataSeq = async () => {
        if (serialState.value !== SerialState.Connected) {
            return; // 未连接时直接返回，避免错误
        }
        try {
            dataSeq.value = await commandGetDataSeq();
        } catch (err) {
            // 仅在连接状态下显示错误
            if (serialState.value === SerialState.Connected) {
                msg.value = `无法获取数据： ${err}`;
            }
        }
    };

    const resetSerialport = async () => {
        if (serialState.value !== SerialState.Connected) {
            return; // 未连接时直接返回，避免错误
        }
        try {
            await commandResetSerialport();
            port.value = ''; // 清空端口
            selected_port.value = ''; // 清空选择的端口
            serialState.value = SerialState.Disconnected; // 设置为未连接
        } catch (err) {
            // 忽略错误，不显示提示
        }
    };

    // 监听 selected_port 的变化
    watch(selected_port, async (newPort) => {
        if (newPort) {
            await setSerialport(newPort); // 更新连接
        }
    });

    return {
        updateSerialports,
        setSerialport,
        updateDataSeq,
        resetSerialport,
        port,
        dataSeq,
        msg,
        selected_port,
        serialState,
        serialports,
    };
});
