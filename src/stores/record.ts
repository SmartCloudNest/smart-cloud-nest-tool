import { ref } from 'vue';
import { commandAppendRecord, commandLastRecord, commandPopRecord, commandRecordsLen, commandSaveRecords, commandResetRecords, CsvRecord } from '../commands';
import { defineStore } from 'pinia';

export type Tags = 'layflat' | 'leftside' | 'rightside';

export const useRecordStore = defineStore('record', () => {
    const recordLength = ref<number>(0);
    const lastRecord = ref<CsvRecord | null>(null);
    const lastDeleted = ref<CsvRecord | null>(null);

    async function updateMetaData() {
        recordLength.value = await commandRecordsLen();
        lastRecord.value = await commandLastRecord();
    }

    async function appendRecord(tag: string, data: number[][]) {
        await commandAppendRecord(tag, data.toString());
        await updateMetaData();
    }

    async function popRecord() {
        lastDeleted.value = await commandPopRecord();
        await updateMetaData();
    }

    async function resetRecords() {
        await commandResetRecords();
        await updateMetaData();
    }

    async function saveRecords(path: string) {
        try {
            await commandSaveRecords(path);
        } catch (err) {
            throw err;
        }
        await updateMetaData();
    }

    return {
        appendRecord,
        popRecord,
        resetRecords,
        saveRecords,

        recordLength,
        lastRecord,
        lastDeleted,
    }
});
