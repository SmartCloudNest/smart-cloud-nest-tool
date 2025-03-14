import { computed, ref } from 'vue';
import { commandAppendRecord, commandLastRecord, commandPopRecord, commandRecordsLen, commandSaveRecords, commandResetRecords, CsvRecord } from '../commands';
import { defineStore } from 'pinia';

<<<<<<< HEAD
<<<<<<< HEAD
export type Tags = 'layflat' | 'leftside' | 'rightside';

export const useRecordStore = defineStore('record', () => {
=======
export const usePortStore = defineStore('port', () => {
>>>>>>> parent of 308d624 (Finished Some works)
=======


export const usePortStore = defineStore('port', () => {
>>>>>>> parent of 00eed2a (current)
    const recordLength = ref<number>(0);
    const lastRecord = ref<CsvRecord | null>(null);
    const

<<<<<<< HEAD
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
=======
>>>>>>> parent of 308d624 (Finished Some works)
});
