import { ref } from "vue";
import { getSerialports, setSerialport, getDataSeq } from "./serial";
import { defineStore } from "pinia";

export const useAppStore = defineStore("app", () => {
    const selectedPort = ref<string>();
    const dataSeq = ref<[number]>();
    const msg = ref<string>();
    const serialports = ref<[number]>();

})