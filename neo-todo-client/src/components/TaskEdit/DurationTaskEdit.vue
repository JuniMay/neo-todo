<script lang="ts">

import { defineComponent, ref } from 'vue';
import { taskKind, taskIcon, deleteDurationTask, DurationTask, updateDurationTask, CommonTask, updateCommonTask, convertToCommonTask, convertToReminderTask } from '../../utils';
import { start } from 'repl';

export default defineComponent({
  name: "DurationTaskEdit",
  setup(props) {
    const id = ref(props.task.id);
    const title = ref(props.task.title);
    const description = ref(props.task.description);
    const status = ref(props.task.status);
    const deadline = ref(props.task.deadline);

    const start_time = ref(props.task.start_time);
    const end_time = ref(props.task.end_time);

    console.log(start_time)
    console.log(end_time)


    async function save() {

      let new_common_task: CommonTask = {
        id: id.value,
        title: title.value,
        kind: 1,
        description: description.value,
        status: status.value,
        deadline: deadline.value,
        category_id: props.task.category_id,
        priority: props.task.priority,
      }

      await updateCommonTask(new_common_task);

      let new_task: DurationTask = {
        id: id.value,
        title: title.value,
        description: description.value,
        status: status.value,
        deadline: deadline.value,
        category_id: props.task.category_id,
        priority: props.task.priority,
        start_time: start_time.value,
        end_time: end_time.value,
      }

      await updateDurationTask(new_task);
      await props.updateCallback();
    }

    return {
      id,
      title,
      description,
      status,
      deadline,
      taskKind,
      taskIcon,
      start_time,
      end_time,
      deleteDurationTask,
      save,
      convertToCommonTask,
      convertToReminderTask
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
  <v-row dense>
    <v-col cols="3">
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
    <v-btn prepend-icon="mdi-content-save" elevation="0" @click="async () => { await save(); }">Save</v-btn>
    <v-btn elevation="0"
      @click="async () => { await convertToCommonTask(task as DurationTask); await updateCallback(); }">
      <v-icon :icon="taskIcon(0)" start></v-icon>
      Common Task
    </v-btn>
    <v-btn elevation="0"
      @click="async () => { await convertToReminderTask(task as DurationTask); await updateCallback(); }">
      <v-icon :icon="taskIcon(2)" start></v-icon>
      Reminder Task
    </v-btn>
    <v-btn prepend-icon="mdi-delete" elevation="0"
      @click="async () => { await deleteDurationTask(id); await updateCallback(); }">Delete</v-btn>

  </v-row>
  <v-row></v-row>
</template>