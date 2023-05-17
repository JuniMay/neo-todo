<script lang="ts">

import { defineComponent, ref } from 'vue';
import { fetchAllLogs, AuditLog } from '../utils';

import { VDataTable } from 'vuetify/labs/VDataTable'


type Headers = InstanceType<typeof VDataTable>['headers']


export default defineComponent({
  name: "LogPage",
  components: {
    VDataTable
  },
  setup() {

    const itemsPerPage = ref(10);

    const headers = ref([
      { title: 'LogID', align: 'start', key: 'log_id' },
      { title: 'TaskID', align: 'start', key: 'task_id' },
      { title: 'Old Status', align: 'start', key: 'old_status' },
      { title: 'New Status', align: 'start', key: 'new_status' },
      { title: 'Date', align: 'start', key: 'audit_date' },
    ] as Headers)

    let allLogs = ref([] as AuditLog[]);

    async function loadLogs() {
      allLogs.value = await fetchAllLogs();

      console.log('allLogs', allLogs);
    }

    loadLogs();

    return {
      headers,
      allLogs,
      loadLogs,
      itemsPerPage
    }
  }
})
</script>

<template>
  <v-card rounded="0">
    <v-toolbar color="#711a5f">
      <v-toolbar-title style="color: white;">NEO TODO</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn class="ma-2" @click="loadLogs()" icon="mdi-sync" style="color: white;"></v-btn>
    </v-toolbar>

    <v-card>
      <v-data-table v-model:items-per-page="itemsPerPage" :headers="headers" :items="allLogs" item-value="log_id"
        elevation="0" style="font-family: monospace;">

      </v-data-table>
    </v-card>

  </v-card>
</template>