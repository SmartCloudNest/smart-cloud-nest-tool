import { invoke } from '@tauri-apps/api/core';

export async function commandSetSerialport(name: string): Promise<void> {
    return await invoke<void>("command_set_serialport", { "name": name });
}

export async function commandGetSerialports(): Promise<string[]> {
    return await invoke<string[]>("command_get_serialports", {});
}

export async function commandGetDataSeq(): Promise<number[]> {
    return await invoke<number[]>("command_get_data_seq", {})
}

export async function commandResetSerialport(): Promise<void> {
    await invoke<void>("command_reset_serialport", {});
}