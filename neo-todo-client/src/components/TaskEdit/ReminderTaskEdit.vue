<script lang="ts">

import { defineComponent, ref } from 'vue';
import { taskKind, taskIcon, deleteReminderTask, ReminderTask, updateReminderTask, CommonTask, updateCommonTask, convertToCommonTask, convertToDurationTask } from '../../utils';

export default defineComponent({
  name: "ReminderTaskEdit",
  setup(props) {
    const id = ref(props.task.id);
    const title = ref(props.task.title);
    const description = ref(props.task.description);
    const status = ref(props.task.status);
    const deadline = ref(props.task.deadline);

    const remind_time = ref(props.task.remind_time);


    async function save() {
      let new_common_task: CommonTask = {
        id: id.value,
        title: title.value,
        kind: 2,
        description: description.value,
        status: status.value,
        deadline: deadline.value,
        category_id: props.task.category_id,
        priority: props.task.priority,
      }

      await updateCommonTask(new_common_task);

      let new_task: ReminderTask = {
        id: id.value,
        title: title.value,
        description: description.value,
        status: status.value,
        deadline: deadline.value,
        category_id: props.task.category_id,
        priority: props.task.priority,
        remind_time: remind_time.value,
      }

      await updateReminderTask(new_task);
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
      remind_time,
      deleteReminderTask,
      save,
      convertToCommonTask,
      convertToDurationTask,
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
    <v-col>
      <v-text-field type="string" v-model="remind_time" label="Remind Time"></v-text-field>
    </v-col>
  </v-row>

  <v-row dense>
    <v-textarea rows="2" type="string" v-model="description" label="Description"></v-textarea>
  </v-row>
  <v-row>
    <v-btn prepend-icon="mdi-cancel" elevation="0">Cancel</v-btn>
    <v-btn prepend-icon="mdi-content-save" elevation="0" @click="async () => { await save(); }">Save</v-btn>
    <v-btn elevation="0"
        @click="async () => { await convertToCommonTask(task as ReminderTask); await updateCallback(); }">
      <v-icon :icon="taskIcon(0)" start></v-icon>
      Common Task
    </v-btn>
    <v-btn elevation="0"
        @click="async () => { await convertToDurationTask(task as ReminderTask); await updateCallback(); }">
      <v-icon :icon="taskIcon(1)" start></v-icon>
      Duration Task
    </v-btn>
    <v-btn prepend-icon="mdi-delete" elevation="0"
      @click="async () => { await deleteReminderTask(id); await updateCallback(); }">Delete</v-btn>

  </v-row>
  <v-row></v-row></template>