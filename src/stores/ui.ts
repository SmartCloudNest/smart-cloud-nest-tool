import { ref } from 'vue';
import { defineStore } from 'pinia';

export const useAppStore = defineStore('app', () => {
    const serialports = ref<string>("");

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

    return {
        updateSerialports,
        updateDataSeq,
        dataSeq,
        serialports,
    };
});
