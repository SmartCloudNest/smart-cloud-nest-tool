import { defineStore } from 'pinia'
import { ref } from 'vue';

function set_serialport(name: string): {

}

export const useSerialStore = defineStore('serial', () => {
    let serialName = ref<string | undefined>(undefined);
    let dataSeq = ref<number | undefined>(undefined);
    let serialports = ref<string | undefined>(undefined);
    let
    return;
})
