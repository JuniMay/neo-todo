<script setup lang="ts">
import { ref } from "vue";
import { getClient } from "@tauri-apps/api/http";
import axios from 'axios';

import TaskItem from "./TaskItem.vue";


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

async function fetchCommonTasks() {
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
  fetchCommonTasks();
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
    <v-toolbar color="#711a5f">
      <v-toolbar-title style="color: white;">NEO TODO</v-toolbar-title>
      <v-toolbar-subtitle style="color: white;">NKU Education Oriented TODO Application</v-toolbar-subtitle>
      <v-spacer></v-spacer>
      <v-btn class="ma-2" @click="fetchCommonTasks()" icon="mdi-sync" style="color: white;"></v-btn>
      <v-btn class="ma-2" @click="openAddDialog()" icon="mdi-plus" style="color: white;"></v-btn>
    </v-toolbar>

    <v-expansion-panels v-for="(task, index) in tasks" :key="index">
      <task-item :task="task" />
    </v-expansion-panels>

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
                    <v-text-field type="date" v-model="selectedTask.deadline" label="Deadline"></v-text-field>
                  </v-col>
                  <v-col>
                    <v-text-field type="time" v-model="selectedTask.deadline" label="Deadline"></v-text-field>
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
