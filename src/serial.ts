import { invoke } from '@tauri-apps/api/core';

export async function setSerialport(name: string): Promise<void> {
    try {
        await invoke<void>("command_set_serialport", {"name": name});
    } 
    catch(err) {
        throw err;
    }
}

export async function getSerialports(): Promise<[number]> {
    try {
        const serialports = await invoke<[number]>("command_get_serialports", {});
        return serialports;
    } 
    catch(err) {
        throw err;
    }
}

export async function getDataSeq(): Promise<[number]> {
    try {
        const dataSeq = await invoke<[number]>("command_get_data_seq", {});
        return dataSeq;
    } 
    catch(err) {
        throw err;
    }
}