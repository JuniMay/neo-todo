
<script lang="ts">
import { defineComponent, ref } from 'vue';
import { taskKind, taskIcon, Tag, fetchCategory, fetchAllCategories, Category } from '../utils';
import TaskEdit from './TaskEdit.vue';

import { DurationTask, ReminderTask, CommonTask, fetchTags, fetchAllTags, addTaskTag, deleteTaskTag, updateCommonTask } from '../utils';

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
    
    const status = ref(props.task.status);
    const deadline = ref(props.task.deadline);
    const category_id = ref(props.task.category_id);

    const tags = ref([] as Tag[]);

    const taskTags = ref([] as Tag[])
    let prevTaskTags = [] as Tag[];

    const kind = ref(0);

    if ("start_time" in props.task) {
      kind.value = 1;
    } else if ("remind_time" in props.task){
      kind.value = 2;
    }

    async function loadTags() {
      tags.value = await fetchAllTags();
      taskTags.value = await fetchTags(id.value);
      prevTaskTags = taskTags.value.slice();
    }

    loadTags();

    const tagSearch = ref("");

    console.log(task);
    console.log(tags);

    async function updateTaskTags() {
      console.log("taskTags", taskTags);
      console.log("prevTaskTags", prevTaskTags);

      let addDiffTags = taskTags.value.filter(tag =>
        prevTaskTags.some(prevTag => tag.id === prevTag.id) === false
      );

      let subDiffTags = prevTaskTags.filter(prevTag =>
        taskTags.value.some(tag => prevTag.id === tag.id) === false
      );

      console.log("addDiffTags", addDiffTags);
      console.log("subDiffTags", subDiffTags);

      for (let i = 0; i < addDiffTags.length; i++) {
        await addTaskTag(id.value, addDiffTags[i].id);
      }
      for (let i = 0; i < subDiffTags.length; i++) {
        await deleteTaskTag(id.value, subDiffTags[i].id);
      }

      prevTaskTags = taskTags.value.slice();
    }


    const allCategories = ref([] as Category[]);
    const category = ref<Category | null>(null);

    async function loadCategory() {
      allCategories.value = await fetchAllCategories();

      if (category_id.value === null) {
        category.value = null;
      } else {
        category.value = await fetchCategory(category_id.value);
      }

      console.log('category', category);
      console.log('allCategories', allCategories);
    }

    async function updateCategory() {
      console.log('new category', category);

      let newCommonTask: CommonTask = {
        id: id.value,
        title: title.value,
        kind: kind.value,
        description: description.value,
        status: status.value,
        deadline: deadline.value,
        category_id: category.value?.id,
        priority: props.task.priority,
      }

      console.log('newCommonTask', newCommonTask);

      await updateCommonTask(newCommonTask);
    }

    loadCategory();

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
      taskTags,
      tagSearch,
      updateTaskTags,
      category,
      allCategories,
      loadCategory,
      updateCategory,
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
        <v-col cols="6" align-self="center">
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
          <v-combobox v-model="taskTags" v-model:search="tagSearch" :items="tags" multiple
            :item-title="item => '#' + item.id + ' ' + item.name" :hide-no-data="false"
            @update:model-value="updateTaskTags" label="Tags" variant="outlined">
            <template v-slot:no-data>
              <v-list-item>
                <v-list-item-title>
                  No tags matching "<strong>{{ tagSearch }}</strong>". Press <kbd>enter</kbd> to create a new one
                </v-list-item-title>
              </v-list-item>
            </template>
            <template v-slot:selection="x">
              <v-chip color="#711a5f" size="x-small">
                #{{ x.item.raw.id }}&nbsp;{{ x.item.raw.name }}
              </v-chip>
            </template>
          </v-combobox>
        </v-col>
        <v-col>
          <v-select v-model="category" :items="allCategories" :item-title="item => '#' + item.id + ' ' + item.name"
            :return-object="true" variant="outlined" label="Category" @update:model-value="updateCategory" clearable>
          </v-select>
        </v-col>
      </v-row>

      <TaskEdit :task="task" :kind="kind" :update-callback="async () => {
        await updateCallback();
      }"></TaskEdit>
    </v-expansion-panel-text>
  </v-expansion-panel>
</template>