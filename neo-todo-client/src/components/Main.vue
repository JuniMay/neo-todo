<script lang="ts">
import { ref } from "vue";
import { getClient } from "@tauri-apps/api/http";

import TaskItem from "./TaskItem.vue";
import TagEdit from "./TagEdit.vue";
import CategoryEdit from "./CategoryEdit.vue";
import LogPage from "./LogPage.vue";

import { CommonTask, createNewTask, fetchAllCommonTasks } from "../utils";
import { defineComponent } from "vue";


export default defineComponent({
  components: {
    TaskItem,
    TagEdit,
    CategoryEdit,
    LogPage,
  },
  setup() {
    const baseUrl = 'http://127.0.0.1:8000';

    const tasks = ref([] as CommonTask[]);
    const selectedTask = ref<CommonTask>();


    async function fetch() {
      tasks.value = await fetchAllCommonTasks();
      console.log(tasks)
    }

    const navPage = ref("pg_tasks");

    fetch();

    return {
      tasks,
      selectedTask,
      fetch,
      createNewTask,
      navPage,
    }
  }
})

</script>

<template>
  <template v-if="navPage === 'pg_tasks'">

    <v-card rounded="0">
      <v-toolbar color="#711a5f">
        <v-toolbar-title style="color: white;">NEO TODO</v-toolbar-title>
        <v-spacer></v-spacer>
        <v-btn class="ma-2" @click="fetch()" icon="mdi-sync" style="color: white;"></v-btn>
        <v-btn class="ma-2" @click="async () => { await createNewTask(); await fetch(); }" icon="mdi-plus"
          style="color: white;"></v-btn>
      </v-toolbar>

      <v-expansion-panels v-for="(task, _) in tasks" :key="JSON.stringify(task)">
        <task-item :task="task" :update-callback="async () => { await fetch(); }" />
      </v-expansion-panels>
    </v-card>
  </template>

  <template v-else-if="navPage === 'pg_tags'">
    <tag-edit></tag-edit>
  </template>

  <template v-else-if="navPage === 'pg_categories'">
    <category-edit></category-edit>
  </template>

  <template v-else-if="navPage === 'pg_log'">
    <log-page></log-page>
  </template>

  <!-- I cannot figure out how to prevent the bottom navigation from blocking contents so I just simply add some rows here.-->
  <v-card rounded="0" elevation="0">
    <v-row>
      <v-col>&nbsp;</v-col>
    </v-row>
    <v-row>
      <v-col>&nbsp;</v-col>
    </v-row>
  </v-card>


  <v-layout>
    <v-bottom-navigation grow v-model="navPage">

      <v-btn style="color: #711a5f;" value="pg_tasks">
        <v-icon>mdi-checkbox-marked-circle-auto-outline</v-icon>

        Tasks
      </v-btn>


      <v-btn style="color: #711a5f;" value="pg_tags">
        <v-icon>mdi-tag</v-icon>

        Tags
      </v-btn>
      <v-btn style="color: #711a5f;" value="pg_categories">
        <v-icon>mdi-shape</v-icon>

        Categories
      </v-btn>

      <v-btn style="color: #711a5f;" value="pg_log">
        <v-icon>mdi-math-log</v-icon>

        Log
      </v-btn>
    </v-bottom-navigation>
  </v-layout>
</template>
