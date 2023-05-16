<script lang="ts">
import { ref } from "vue";
import { getClient } from "@tauri-apps/api/http";
import axios from 'axios';

import TaskItem from "./TaskItem.vue";
import { CommonTask } from "../utils";
import { defineComponent } from "vue";


export default defineComponent({
  components: {
    TaskItem,
  },
  setup() {
    enum DialogKind {
      Add,
      Edit,
      None,
    }

    const base_url = 'http://127.0.0.1:8000';

    const tasks = ref([] as CommonTask[]);
    const selectedTask = ref<CommonTask>();

    const currDialogKind = ref(DialogKind.None);
    const isDialogOpen = ref(false);

    async function fetchCommonTasks() {

      console.log('fectched');

      const client = await getClient();
      const response = await client.request({
        method: 'GET',
        url: `${base_url}/fetch/all-common-tasks`,
      });


      tasks.value = response.data as CommonTask[];
      console.log(tasks)
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

    return {
      tasks,
      selectedTask,
      fetchCommonTasks,
      openAddDialog,
      isDialogOpen,
      currDialogKind,
      DialogKind,
    }
  }
})

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

    <v-expansion-panels v-for="(task, _) in tasks" :key="'(' + task.id + ',' + task.title + ',' + task.status + ')'">
      <task-item :task="task" :update-callback="async () => { await fetchCommonTasks(); }" />
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
          <v-btn color="blue darken-1" text="Save">Save</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-card>
</template>
