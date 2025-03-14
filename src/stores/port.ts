import { computed, ref } from 'vue';
import { commandGetSerialports, commandConnectPort, commandGetDataGrid, commandResetPort } from '../commands';
import { defineStore } from 'pinia';

export enum SerialState {
    Disconnected,
    Connecting,
    Connected,

}

export const usePortStore = defineStore('port', () => {
    const dataGrid = ref<number[][]>(new Array(10).fill(new Array(16).fill(0)));
    const serialports = ref<string[]>([]);
    const port = ref<string | null>(null);
    const serialState = ref<SerialState>(SerialState.Disconnected);

    async function updateSerialports() {
        serialports.value = await commandGetSerialports();
    }

    async function updateDataGrid() {
        if (!port.value) {
            return;
        }
        try {
            dataGrid.value = await commandGetDataGrid();
        } catch (err) {
            serialState.value = SerialState.Disconnected;
            throw new Error(`${err}`);
        }
    };

    async function resetSerialport() {
        await commandResetPort();
    };

    async function connectPort() {
        if (!port.value) {
            return;
        }
        try {
            serialState.value = SerialState.Connecting;
            await commandResetPort();
            await commandConnectPort(port.value);
            serialState.value = SerialState.Connected;
        } catch (err) {
            serialState.value = SerialState.Disconnected;
            throw new Error(`${err}`);
        }
    }

    const isDisconnected = computed(() => serialState.value === SerialState.Disconnected);
    const isConnecting = computed(() => serialState.value === SerialState.Connecting);
    const isConnected = computed(() => serialState.value === SerialState.Connected);

    return {
        updateSerialports,
        updateDataGrid,
        resetSerialport,
        connectPort,

        dataGrid,
        serialports,
        serialState,
        port,
        isDisconnected,
        isConnecting,
        isConnected,
    };
});
