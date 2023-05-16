
import axios from "axios";
import { getClient } from "@tauri-apps/api/http";
export const base_url = 'http://127.0.0.1:8000';

const post_headers = {
  'Content-Type': 'application/x-www-form-urlencoded',
};

export interface CommonTask {
  id: number,
  title: string,
  description?: string,
  deadline?: string,
  priority?: number,
  status?: string,
  category_id?: number,

  kind: number,
}

export interface DurationTask {
  id: number,
  title: string,
  description?: string,
  deadline?: string,
  priority?: number,
  status?: string,
  category_id?: number,

  start_time: string,
  end_time: string
}

export interface ReminderTask {
  id: number,
  title: string,
  description?: string,
  deadline?: string,
  priority?: number,
  status?: string,
  category_id?: number,

  remind_time: string,
}

export function taskKind(kind: number): string {
  if (kind === 0) {
    return 'Common Task'
  } else if (kind === 1) {
    return 'Duration Task'
  } else if (kind === 2) {
    return 'Reminder Task'
  }
  return ''
}

export function taskIcon(kind: number): string {
  if (kind === 0) {
    return 'mdi-timelapse'
  } else if (kind === 1) {
    return 'mdi-timer-sand'
  } else if (kind === 2) {
    return 'mdi-alert-box-outline'
  }
  return 'mdi-altimeter'
}

export function updateCommonTask(task: CommonTask): void {
  const request_url = `${base_url}/update/update-common-task`
  axios.post(request_url, JSON.stringify(task), { headers: post_headers })
}

export function updateDurationTask(task: DurationTask): void {
  const request_url = `${base_url}/update/update-duration-task`;
  const data = {
    id: task.id,
    start_time: task.start_time,
    end_time: task.end_time
  }
  axios.post(request_url, JSON.stringify(data), { headers: post_headers })
}

export function updateReminderTask(task: ReminderTask): void {
  const request_url = `${base_url}/update/update-reminder-task`;
  const data = {
    id: task.id,
    remind_time: task.remind_time
  }
  axios.post(request_url, JSON.stringify(data), { headers: post_headers })
}

export async function fetchCommonTask(id: number): Promise<CommonTask> {
  const client = await getClient();
  const request_url = `${base_url}/fetch/common-task?id=${id}`;
  const response = await client.request({
    method: 'GET',
    url: request_url,
  });
  return response.data as CommonTask;
}

export async function fetchDurationTask(id: number): Promise<DurationTask> {
  const client = await getClient();
  const request_url = `${base_url}/fetch/duration-task?id=${id}`;
  const response = await client.request({
    method: 'GET',
    url: request_url,
  });
  return response.data as DurationTask;
}

export async function fetchReminderTask(id: number): Promise<ReminderTask> {
  const client = await getClient();
  const request_url = `${base_url}/fetch/reminder-task?id=${id}`;
  const response = await client.request({
    method: 'GET',
    url: request_url,
  });
  return response.data as ReminderTask;
}

export async function deleteCommonTask(id: number): Promise<void> {
  const client = await getClient();
  const request_url = `${base_url}/delete/common-task?id=${id}`;
  await client.request({
    method: 'GET',
    url: request_url,
  });
}

export async function deleteReminderTask(id: number): Promise<void> {
  const client = await getClient();
  const request_url = `${base_url}/delete/reminder-task?id=${id}`;
  await client.request({
    method: 'GET',
    url: request_url,
  });
}

export async function deleteDurationTask(id: number): Promise<void> {
  const client = await getClient();
  const request_url = `${base_url}/delete/duration-task?id=${id}`;
  await client.request({
    method: 'GET',
    url: request_url,
  });
}