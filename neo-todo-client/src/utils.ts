
import { Body, getClient } from "@tauri-apps/api/http";
export const base_url = 'http://127.0.0.1:8000';

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

export interface Tag {
  id: number,
  name: string,
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


export async function fetchAllCommonTasks() {

  console.log('fectched');

  const client = await getClient();
  const response = await client.request({
    method: 'GET',
    url: `${base_url}/fetch/all-common-tasks`,
  });

  console.log("fetchAllCommonTasks", response);

  return response.data as CommonTask[];
}

export async function createNewTask() {
  const client = await getClient();
  const request_url = `${base_url}/create/common-task`;

  const data: CommonTask = {
    id: 0,
    title: "NEW TASK",
    kind: 0,
    status: "TODO",
    deadline: (new Date()).toISOString(),
  }

  const response = await client.request({
    method: 'POST',
    url: request_url,
    body: Body.json(data),
  });
  console.log(response);
}

export async function updateCommonTask(task: CommonTask) {

  const client = await getClient();
  const request_url = `${base_url}/update/update-common-task`;

  const response = await client.request({
    method: 'POST',
    url: request_url,
    body: Body.json(task),
  });
  console.log(response);
}

export async function updateDurationTask(task: DurationTask) {

  const client = await getClient();
  const request_url = `${base_url}/update/update-duration-task`;
  const data = {
    id: task.id,
    start_time: task.start_time,
    end_time: task.end_time
  }

  const response = await client.request({
    method: 'POST',
    url: request_url,
    body: Body.json(data),
  });

  console.log(response);
}

export async function updateReminderTask(task: ReminderTask) {
  const client = await getClient();
  const request_url = `${base_url}/update/update-reminder-task`;
  const data = {
    id: task.id,
    remind_time: task.remind_time
  }
  const response = await client.request({
    method: 'POST',
    url: request_url,
    body: Body.json(data),
  });
  console.log(response);
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

export async function fetchAllTags(): Promise<Array<Tag>> {
  const client = await getClient();
  const request_url = `${base_url}/fetch/all-tags`;
  const response = await client.request({
    method: 'GET',
    url: request_url,
  });
  return response.data as Array<Tag>;
}

export async function fetchTags(task_id: number): Promise<Array<Tag>> {
  const client = await getClient();
  const request_url = `${base_url}/fetch/tags?task_id=${task_id}`;
  const response = await client.request({
    method: 'GET',
    url: request_url,
  });
  return response.data as Array<Tag>;
}

export async function addTag(tag: Tag) {
  const client = await getClient();
  const requestUrl = `${base_url}/create/tag`;
  const response = await client.request({
    method: 'POST',
    url: requestUrl,
    body: Body.json(tag)
  });
  console.log('addTag', response);
}

export async function deleteTag(tag_id: number) {
  const client = await getClient();
  const requestUrl = `${base_url}/delete/tag?id=${tag_id}`;
  const response = await client.request({
    method: 'GET',
    url: requestUrl,
  });
  console.log('deletTag', response);
}

export async function addTaskTag(task_id: number, tag_id: number) {
  const client = await getClient();
  const request_url = `${base_url}/update/add-task-tag`;
  const response = await client.request({
    method: 'POST',
    url: request_url,
    body: Body.json({
      task_id: task_id,
      tag_id: tag_id
    })
  });
  console.log(response);
}

export async function deleteTaskTag(task_id: number, tag_id: number) {
  const client = await getClient();
  const request_url = `${base_url}/delete/task-tag?task_id=${task_id}&tag_id=${tag_id}`;
  const response = await client.request({
    method: 'GET',
    url: request_url,
  });
  console.log(response);
}

export async function convertToCommonTask(task: DurationTask | ReminderTask | CommonTask) {
  const client = await getClient();
  const request_url = `${base_url}/update/to-common-task`;
  const data = {
    id: task.id,
  }

  console.log("convert")
  console.log(data)

  const response = await client.request({
    method: 'POST',
    url: request_url,
    body: Body.json(data),
  });

  console.log(response);

}
export async function convertToDurationTask(task: DurationTask | ReminderTask | CommonTask) {
  const client = await getClient();
  const request_url = `${base_url}/update/to-duration-task`;
  const data = {
    id: task.id,
    start_time: task.deadline || (new Date()).toISOString(),
    end_time: task.deadline || (new Date()).toISOString()
  }

  console.log("convert")
  console.log(data)

  const response = await client.request({
    method: 'POST',
    url: request_url,
    body: Body.json(data),
  });

  console.log(response);

}
export async function convertToReminderTask(task: DurationTask | ReminderTask | CommonTask) {
  const client = await getClient();
  const request_url = `${base_url}/update/to-reminder-task`;
  const data = {
    id: task.id,
    remind_time: task.deadline || (new Date()).toISOString()
  }

  console.log("convert")
  console.log(data)

  const response = await client.request({
    method: 'POST',
    url: request_url,
    body: Body.json(data),
  });

  console.log(response);
}