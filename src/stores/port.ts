import { ref } from 'vue';
import { commandGetSerialports, commandConnectPort, commandGetDataSeq, commandResetPort } from '../commands';
import { defineStore } from 'pinia';

export enum SerialState {
    Disconnected,
    Connecting,
    Connected,

}

export const usePortStore = defineStore('port', () => {
    const dataSeq = ref<number[]>([]);
    const serialports = ref<string[]>([]);
    const port = ref<string | null>(null);
    const serialState = ref<SerialState>(SerialState.Disconnected);

    async function updateSerialports(): Promise<void> {
        serialports.value = await commandGetSerialports();
    }

    async function updateDataSeq(): Promise<void> {
        if (!port.value) {
            return;
        }
        try {
            dataSeq.value = await commandGetDataSeq();
        } catch (err) {
            serialState.value = SerialState.Disconnected;
            throw new Error(`${err}`);
        }
    };

    async function resetSerialport(): Promise<void> {
        await commandResetPort();
    };

    async function connectPort(name: string): Promise<void> {
        try {
            serialState.value = SerialState.Connecting;
            await commandConnectPort(name);
            port.value = name;
            serialState.value = SerialState.Connected;
        } catch (err) {
            throw new Error(`${err}`);
        }
    }

    async function reconnect(): Promise<void> {
        if (!port.value) {
            return;
        }
        try {
            serialState.value = SerialState.Connecting;
            await commandConnectPort(port.value);
            serialState.value = SerialState.Connected;
        } catch (err) {
            throw new Error(`${err}`);
        }
    }

    return {
        updateSerialports,
        updateDataSeq,
        resetSerialport,
        connectPort,
        reconnect,
        dataSeq,
        serialports,
        serialState,
        port,
    };
});
