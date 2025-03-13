import { ref } from 'vue';
import { commandAppendRecord, commandLastRecord, commandPopRecord, commandRecordsLen, commandSaveRecords, commandResetRecords, CsvRecord } from '../commands';
import { defineStore } from 'pinia';



export const usePortStore = defineStore('port', () => {
    const recordLength = ref<number>(0);
    const lastRecord = ref<CsvRecord | null>(null);
    const lastDeleted = ref<CsvRecord | null>(null);

    async function updateMetaData() {
        recordLength.value = await commandRecordsLen();
        lastRecord.value = await commandLastRecord();
    }

    async function appendRecord(tag: string, data: number[][]) {
        commandAppendRecord(tag, JSON.stringify(data).toString());
        try {
            await updateMetaData();
        } catch (err) {
            throw err;
        }
    }

    async function popRecord() {
        lastDeleted.value = await commandPopRecord();
        try {
            await updateMetaData();
        } catch (err) {
            throw err;
        }
    }

    async function resetRecords() {
        await commandResetRecords();
        try {
            await updateMetaData();
        } catch (err) {
            throw err;
        }
    }

    async function saveRecords(path: string) {
        await commandSaveRecords(path);
        try {
            await updateMetaData();
        } catch (err) {
            throw err;
        }
    }

    return {
        appendRecord,
        popRecord,
        resetRecords,
        saveRecords,

        recordLength,
        lastRecord,
        lastDeleted
    }
});
