<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { getClient, ResponseType } from "@tauri-apps/api/http";

const name = ref("");
const tasks = ref<any[]>([]);

async function common_tasks() {
  const client = await getClient();
  const response = await client.request({
    method: 'GET',
    url: 'http://127.0.0.1:8000/',
  });

  tasks.value = response.data as any[];
}

</script>

<template>
  <v-card rounded="0">
    <v-toolbar color="purple">
      <v-btn icon="mdi-menu"></v-btn>
      <v-toolbar-title>NEO TODO</v-toolbar-title>
      <v-toolbar-subtitle>NKU Education Oriented TODO Application</v-toolbar-subtitle>
      <v-spacer></v-spacer>
      <!-- <v-text-field v-model="name" placeholder="Enter a name..." /> -->
      <v-btn type="button" @click="common_tasks()" icon="mdi-magnify"></v-btn>
    </v-toolbar>
    <v-list v-for="(task, index) in tasks" :key="index" class="grid-container">
      <v-list-item>
        <template v-slot:prepend="{ isActive }">
          <v-list-item-action start>
            <v-checkbox-btn :model-value="isActive"></v-checkbox-btn>
          </v-list-item-action>
          <!-- <v-chip>{{ task.id }}</v-chip> -->
        </template>
        <v-spacer></v-spacer>
        <v-list-item-title>{{ task.title }}</v-list-item-title>
        <v-list-item-subtitle>{{ task.description }}</v-list-item-subtitle>
      </v-list-item>
    </v-list>
  </v-card>
</template>
