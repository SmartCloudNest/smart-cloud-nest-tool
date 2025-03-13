import { computed, ref } from 'vue';
import { commandAppendRecord, commandLastRecord, commandPopRecord, commandRecordsLen, commandSaveRecords, commandResetRecords, CsvRecord } from '../commands';
import { defineStore } from 'pinia';

export const usePortStore = defineStore('port', () => {
    const recordLength = ref<number>(0);
    const lastRecord = ref<CsvRecord | null>(null);
    const

});
