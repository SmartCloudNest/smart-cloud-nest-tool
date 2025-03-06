import { ref } from "vue";
import { commandGetSerialports, commandSetSerialport, commandGetDataSeq } from "./serial";
import { defineStore } from "pinia";

export const useAppStore = defineStore("app", () => {
    const port = ref<string>();
    const dataSeq = ref<[number]>();
    const msg = ref<string>();
    const serialports = ref<[number]>();
    const updateSerialports = async ()=>{
        try {
            serialports.value = await commandGetSerialports();
        }
        catch(err) {
            msg.value = `获取串口列表失败： ${ err }`;
        }
    }
    const setSerialport = async (name: string)=>{
        try {
            await commandSetSerialport(name);
            port.value = name;
        }
        catch(err) {
            msg.value = `无法连接到串口： ${err}`;
        }
    }
    const updateDataSeq = async ()=>{
        try {
            dataSeq.value = await commandGetDataSeq();
        }
        catch(err) {
            msg.value = `无法获取数据： ${err}`;
            port.value = undefined;
        }
    }
    return { updateSerialports, setSerialport, updateDataSeq };
})