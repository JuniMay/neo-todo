<script lang="ts">
import { ref } from "vue";
import { getClient } from "@tauri-apps/api/http";
import axios from 'axios';

import TaskItem from "./TaskItem.vue";
import { CommonTask, createNewTask } from "../utils";
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
      createNewTask
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
      <v-btn class="ma-2" @click="async () => { await createNewTask(); await fetchCommonTasks(); }" icon="mdi-plus"
        style="color: white;"></v-btn>
    </v-toolbar>

    <v-expansion-panels v-for="(task, _) in tasks"
      :key="'(' + task.id + ',' + task.title + ',' + task.status + ',' + task.kind + ')'">
      <task-item :task="task" :update-callback="async () => { await fetchCommonTasks(); }" />
    </v-expansion-panels>
  </v-card>
</template>
