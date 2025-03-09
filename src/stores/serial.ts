import { onUnmounted, ref } from 'vue';
import { commandGetSerialports, commandConnectPort, commandGetDataSeq, commandResetPort } from '../commands';
import { defineStore } from 'pinia';

enum SerialState {
    Disconnected,
    Connecting,
    Connected,
    Retry,

}

export const useAppStore = defineStore('app', () => {
    const dataSeq = ref<number[]>([]);
    const serialports = ref<string[]>([]);
    const port = ref<string | null>(null);
    const serialState = ref<SerialState>(SerialState.Disconnected);

    async function updateSerialports(): Promise<void> {
        serialports.value = await commandGetSerialports();
    }

    async function updateDataSeq(): Promise<void | Error> {
        try {
            dataSeq.value = await commandGetDataSeq();
        } catch (err) {

            return new Error(`${err}`);
        }
    };

    async function resetSerialport(): Promise<void> {
        await commandResetPort();
        port.value = null;
    };

    async function connectPort(name: string): Promise<void | Error> {
        try {
            await commandConnectPort(name);
            port.value = name;
        } catch (err) {
            return new Error(`${err}`);
        }
    }

    const intervalId = setInterval(updateSerialports, 500);

    onUnmounted(() => {
        clearInterval(intervalId);
    })

    return {
        updateSerialports,
        updateDataSeq,
        resetSerialport,
        connectPort,
        dataSeq,
        serialports,
    };
});
