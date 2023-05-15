<script setup lang="ts">
import { ref } from "vue";
import { getClient } from "@tauri-apps/api/http";
import axios from 'axios';

interface Task {
  id: number,
  title: string,
  description?: string,
  deadline?: string,
  priority?: number,
  status?: string,
  category_id?: number,
}

interface CommonTask {
  id: number,
  title: string,
  description?: string,
  deadline?: string,
  priority?: number,
  status?: string,
  category_id?: number,

  kind: number,
}

enum DialogKind {
  Add,
  Edit,
  None,
}

const base_url = 'http://127.0.0.1:8000';

const tasks = ref<CommonTask[]>([]);
const selectedTask = ref<CommonTask>();

const currDialogKind = ref(DialogKind.None);
const isDialogOpen = ref(false);

async function fetch_common_tasks() {
  const client = await getClient();
  const response = await client.request({
    method: 'GET',
    url: `${base_url}/fetch/all-common-tasks`,
  });

  tasks.value = response.data as CommonTask[];
}

function openEditDialog(task: CommonTask) {
  selectedTask.value = { ...task };
  currDialogKind.value = DialogKind.Edit;
  isDialogOpen.value = true;
}

function openAddDialog() {
  selectedTask.value = {
    id: 0,
    title: "",
    deadline: Date.now().toString(),
    kind: 0,
  };
  currDialogKind.value = DialogKind.Add;
  isDialogOpen.value = true;
}

async function saveTask() {
  if (currDialogKind.value == DialogKind.Edit) {
    axios.post(`${base_url}/update/update-common-task`, JSON.stringify(selectedTask.value), {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      }
    })
  } else if (currDialogKind.value == DialogKind.Add) {
    axios.post(`${base_url}/create/common-task`, JSON.stringify(selectedTask.value), {
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded'
      }
    })
  }
  currDialogKind.value = DialogKind.None;
  isDialogOpen.value = false;
  fetch_common_tasks();
}

function getKind(kind: number) {
  switch (kind) {
    case 0:
      return 'Common';
    case 1:
      return 'Duration';
    case 2:
      return 'Reminder';
    default:
      return '';
  }
}

</script>

<template>
  <v-card rounded="0">
    <v-toolbar color="black">
      <v-toolbar-title> NEO TODO</v-toolbar-title>
      <v-toolbar-subtitle>NKU Education Oriented TODO Application</v-toolbar-subtitle>
      <v-spacer></v-spacer>
      <v-btn class="ma-2" @click="fetch_common_tasks()" icon="mdi-sync"></v-btn>
      <v-btn class="ma-2" @click="openAddDialog()" icon="mdi-plus"></v-btn>
    </v-toolbar>

    <v-list v-for="(task, index) in tasks" :key="index">
      <v-list-item @click="openEditDialog(task)">
        <v-row>
          <v-col>
            <v-list-item-title>{{ task.title }}</v-list-item-title>
            <v-list-item-subtitle class="mb-2">{{ task.description }}</v-list-item-subtitle>
            <v-chip>{{ task.status }}</v-chip>
            <v-list-item-subtitle>{{ getKind(task.kind) }}, Priority: {{ task.priority }}</v-list-item-subtitle>
            <v-list-item-subtitle>{{ (new Date(task.deadline as string)) }}</v-list-item-subtitle>
          </v-col>
        </v-row>
      </v-list-item>
    </v-list>

    <v-dialog v-model="isDialogOpen" max-width="500px" v-if="selectedTask">
      <v-card>
        <v-card-title>
          <span class="text-h5">Edit Task</span>
        </v-card-title>
        <v-card-text>
          <v-container>
            <v-row>
              <v-col>
                <v-text-field v-model="selectedTask.title" label="Title"></v-text-field>
                <v-text-field v-model="selectedTask.description" label="Description"></v-text-field>

                <v-row>

                  <v-col>
                    <v-text-field type="date" v-model="selectedTask.deadline"
                      label="Deadline"></v-text-field>
                  </v-col>
                  <v-col>
                    <v-text-field type="time" v-model="selectedTask.deadline"
                      label="Deadline"></v-text-field>
                  </v-col>
                </v-row>
                <v-text-field type="number" v-model="selectedTask.priority" label="Priority"></v-text-field>
                <v-text-field v-model="selectedTask.status" label="Status"></v-text-field>
              </v-col>
            </v-row>
          </v-container>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="blue darken-1" text="Close"
            @click="currDialogKind = DialogKind.None; isDialogOpen = false"></v-btn>
          <v-btn color="blue darken-1" text="Save" @click="saveTask()">Save</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-card>
</template>
