<script lang="ts">


import { defineComponent, ref } from 'vue';
import CommonTaskEdit from './TaskEdit/CommonTaskEdit.vue';
import DurationTaskEdit from './TaskEdit/DurationTaskEdit.vue'
import ReminderTaskEdit from './TaskEdit/ReminderTaskEdit.vue'

export default defineComponent({
  name: "TaskEdit",
  components: {
    CommonTaskEdit,
    DurationTaskEdit,
    ReminderTaskEdit
  },
  setup() {

  },
  props: {
    task: {
      type: Object,
      required: true
    },
    kind: {
      type: Number,
      required: true
    },
    updateCallback: {
      type: Function,
      required: true
    }
  }
})

</script>

<template>
  <template v-if="kind === 0">
    <CommonTaskEdit :task="task" :update-callback="() => { updateCallback(); }"></CommonTaskEdit>
  </template>
  <template v-else-if="kind === 1">
    <DurationTaskEdit :task="task" :update-callback="() => { updateCallback(); }"></DurationTaskEdit>
  </template>
  <template v-else-if="kind === 2">
    <ReminderTaskEdit :task="task" :update-callback="() => { updateCallback(); }"></ReminderTaskEdit>
  </template>
  <template v-else>
    <CommonTaskEdit :task="task" :update-callback="() => { updateCallback(); }"></CommonTaskEdit>
  </template>
</template>