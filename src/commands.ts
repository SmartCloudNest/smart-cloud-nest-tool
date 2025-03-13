import { invoke } from '@tauri-apps/api/core';

export interface CsvRecord {
    time: number,
    tag: string,
    data: string,
}

export async function commandConnectPort(name: string) {
    return await invoke<void>('command_connect_port', { 'name': name });
}

export async function commandGetSerialports(): Promise<string[]> {
    return await invoke<string[]>('command_get_serialports', {});
}

export async function commandGetDataGrid(): Promise<number[][]> {
    return await invoke<number[][]>('command_get_data_grid', {})
}

export async function commandResetPort() {
    return await invoke<void>('command_reset_port', {});
}

export async function commandAppendRecord(tag: string, data: string): Promise<void> {
    return await invoke<void>('command_append_record', { 'tag': tag, 'data': data });
}

export async function commandLastRecord(): Promise<CsvRecord | null> {
    return await invoke<CsvRecord | null>('command_last_record', {});
}

export async function commandPopRecord(): Promise<CsvRecord | null> {
    return await invoke<CsvRecord | null>('command_pop_record', {});
}

export async function commandSaveRecords(path: string): Promise<void> {
    return await invoke<void>('command_save_records', { 'path': path });
}

export async function commandRecordsLen(): Promise<number> {
    return await invoke<number>('command_records_len', {});
}

export async function commandResetRecords(): Promise<void> {
    return await invoke<void>('command_reset_records', {});
}