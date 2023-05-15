
<script lang="ts">
import { defineComponent, ref } from 'vue';
import { taskKind, taskIcon } from '../utils';
import TaskEdit from './TaskEdit.vue';

export default defineComponent({
  name: 'TodoItem',
  components: {
    TaskEdit
  },
  setup(props) {
    const task = props.task;

    const id = ref(props.task.id);
    const title = ref(props.task.title);
    const description = ref(props.task.description);
    const kind = ref(props.task.kind);
    const status = ref(props.task.status);
    const deadline = ref(props.task.deadline);

    return {
      task,
      id,
      title,
      description,
      kind,
      status,
      deadline,
      taskKind,
      taskIcon,
    }
  },
  props: {
    task: {
      type: Object,
      required: true,
    }
  }
})
</script>

<template>
  <v-expansion-panel>
    <v-expansion-panel-title>
      <v-row>
        <v-col cols="3">
          <v-chip label color="#711a5f" style="width: 10.5em;">
            <v-icon>{{ taskIcon(kind) }}</v-icon>
            {{ taskKind(kind) }}
          </v-chip>
        </v-col>
        <v-col cols="6">
          <div class="font-weight-bold text-h6">
            <v-icon color="#711a5f">mdi-checkbox-marked-circle-auto-outline </v-icon>
            &nbsp;
            {{ title }}
          </div>
        </v-col>
        <v-col cols="3">
          <v-chip color="#711a5f">
            <v-icon>mdi-list-status</v-icon>
            {{ status }}
          </v-chip>
        </v-col>
      </v-row>
    </v-expansion-panel-title>

    <v-expansion-panel-text>
      <TaskEdit :task="task"></TaskEdit>
    </v-expansion-panel-text>
  </v-expansion-panel>
</template>