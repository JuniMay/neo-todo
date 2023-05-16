<script lang="ts">

import { defineComponent, ref } from 'vue';
import { taskKind, taskIcon, deleteDurationTask } from '../../utils';

export default defineComponent({
  name: "DurationTaskEdit",
  setup(props) {
    const id = ref(props.task.id);
    const title = ref(props.task.title);
    const description = ref(props.task.description);
    const kind = ref(props.task.kind);
    const status = ref(props.task.status);
    const deadline = ref(props.task.deadline);

    const start_time = ref(props.task.start_time);
    const end_time = ref(props.task.end_time);

    console.log(start_time)
    console.log(end_time)

    return {
      id,
      title,
      description,
      kind,
      status,
      deadline,
      taskKind,
      taskIcon,
      start_time,
      end_time,
      deleteDurationTask
    }

  },
  props: {
    task: {
      type: Object,
      required: true,
    },
    updateCallback: {
      type: Function,
      required: true,
    }
  }
})

</script>

<template>
  <v-row>
    <v-col></v-col>
  </v-row>
  <v-row dense>
    <v-col cols="2">
      <v-text-field type="string" v-model="status" label="Status"></v-text-field>
    </v-col>
    <v-col>
      <v-text-field type="string" v-model="title" label="Title"></v-text-field>
    </v-col>
  </v-row>

  <v-row dense>
    <v-col>
      <v-text-field type="string" v-model="deadline" label="Deadline"></v-text-field>
    </v-col>
    <!-- TODO: support edit of end/start time -->
    <v-col>
      <v-text-field type="string" v-model="start_time" label="Start Time"></v-text-field>
    </v-col>
    <v-col>
      <v-text-field type="string" v-model="end_time" label="End Time"></v-text-field>
    </v-col>
  </v-row>

  <v-row dense>
    <v-textarea rows="2" type="string" v-model="description" label="Description"></v-textarea>
  </v-row>
  <v-row>
    <v-btn prepend-icon="mdi-cancel" elevation="0">Cancel</v-btn>
    <v-btn prepend-icon="mdi-content-save" elevation="0">Save</v-btn>

    <v-menu>
      <template v-slot:activator="{ props }">
        <v-btn v-bind="props" elevation="0" prepend-icon="mdi-swap-horizontal">
          Change To
        </v-btn>
      </template>

      <v-card>
        <v-card-text>
          <v-btn elevation="0">
            <v-icon :icon="taskIcon(0)" start></v-icon>
            Common Task
          </v-btn>
          <v-btn elevation="0">
            <v-icon :icon="taskIcon(2)" start></v-icon>
            Reminder Task
          </v-btn>
        </v-card-text>
      </v-card>
    </v-menu>
    <v-btn prepend-icon="mdi-delete" elevation="0"
      @click="async () => { await deleteDurationTask(id); await updateCallback(); }">Delete</v-btn>

  </v-row>
  <v-row></v-row>
</template>