
<script lang="ts">
import { defineComponent, ref } from 'vue';
import { taskKind, taskIcon, Tag } from '../utils';
import TaskEdit from './TaskEdit.vue';

import { fetchDurationTask, fetchReminderTask, fetchTags, fetchAllTags, addTaskTag, deleteTaskTag } from '../utils';

export default defineComponent({
  name: 'TodoItem',
  components: {
    TaskEdit
  },
  setup(props) {
    const task = ref(props.task);

    const id = ref(props.task.id);
    const title = ref(props.task.title);
    const description = ref(props.task.description);
    const kind = ref(props.task.kind);
    const status = ref(props.task.status);
    const deadline = ref(props.task.deadline);

    async function patchTask() {
      if (kind.value === 1) {
        task.value = await fetchDurationTask(id.value);
      } else if (kind.value === 2) {
        task.value = await fetchReminderTask(id.value);
      }
    }

    patchTask();

    const tags = ref([] as Tag[]);

    const task_tags = ref([] as Tag[])
    let prev_task_tags = [] as Tag[];

    async function loadTags() {
      tags.value = await fetchAllTags();
      task_tags.value = await fetchTags(id.value);
      prev_task_tags = task_tags.value.slice();
    }

    loadTags();

    const tag_search = ref("");

    console.log(task);
    console.log(tags);

    async function updateTaskTags() {
      console.log("task_tags", task_tags);
      console.log("prev_task_tags", prev_task_tags);

      let add_diff_tags = task_tags.value.filter(tag =>
        prev_task_tags.some(prev_tag => tag.id === prev_tag.id) === false
      );

      let sub_diff_tags = prev_task_tags.filter(prev_tag =>
        task_tags.value.some(tag => prev_tag.id === tag.id) === false
      );

      console.log("add_diff_tags", add_diff_tags);
      console.log("sub_diff_tags", sub_diff_tags);

      for (let i = 0; i < add_diff_tags.length; i++) {
        await addTaskTag(id.value, add_diff_tags[i].id);
      }
      for (let i = 0; i < sub_diff_tags.length; i++) {
        await deleteTaskTag(id.value, sub_diff_tags[i].id);
      }

      prev_task_tags = task_tags.value.slice();
    }

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
      tags,
      task_tags,
      tag_search,
      updateTaskTags
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
  <v-expansion-panel>
    <v-expansion-panel-title>
      <v-row>
        <v-col cols="4">
          <v-row dense>
            <v-col>
              <!-- <v-chip label color="#711a5f" style="width: 3.5em;"> -->
              <!-- {{ id }} -->
              <!-- </v-chip> -->
              <!-- &nbsp; -->
              <v-chip label color="#711a5f" style="width: 10.5em;">
                <v-icon>{{ taskIcon(kind) }}</v-icon>
                {{ taskKind(kind) }}
              </v-chip>
            </v-col>
          </v-row>
          <v-row dense>
            <v-col>
              <v-chip color="#711a5f">
                <v-icon>mdi-list-status</v-icon>
                {{ status }}
              </v-chip>
            </v-col>
          </v-row>
        </v-col>
        <v-col cols="6">
          <v-row dense>
            <v-col>
              <div class="font-weight-bold text-h6">
                <v-icon color="#711a5f">mdi-checkbox-marked-circle-auto-outline</v-icon>
                &nbsp;
                {{ title }}
              </div>
            </v-col>
          </v-row>
          <v-row dense>
            <v-col>
              <div class="font-weight-thin text-disabled text-caption">
                <p class="text-justify">{{ description }}</p>
              </div>
            </v-col>
          </v-row>
        </v-col>
      </v-row>
    </v-expansion-panel-title>

    <v-expansion-panel-text>
      <v-row>
        <v-col></v-col>
      </v-row>
      <v-row>
        <v-col>
          <v-combobox v-model="task_tags" v-model:search="tag_search" :items="tags" multiple
            :item-title="item => '#' + item.id + ' ' + item.name" :hide-no-data="false"
            @update:model-value="updateTaskTags">
            <template v-slot:no-data>
              <v-list-item>
                <v-list-item-title>
                  No tags matching "<strong>{{ tag_search }}</strong>". Press <kbd>enter</kbd> to create a new one
                </v-list-item-title>
              </v-list-item>
            </template>
            <template v-slot:selection="x">
              <v-chip color="#711a5f">
                #{{ x.item.raw.id }}&nbsp;{{ x.item.raw.name }}
              </v-chip>
            </template>
          </v-combobox>
        </v-col>
      </v-row>

      <TaskEdit :task="task" :kind="kind" :update-callback="() => { updateCallback(); }"></TaskEdit>
    </v-expansion-panel-text>
  </v-expansion-panel>
</template>