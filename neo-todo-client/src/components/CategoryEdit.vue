<script lang="ts">

import { defineComponent, ref } from 'vue';
import { Category, fetchAllCategories, deleteCategory, createCategory } from '../utils';


export default defineComponent({
  name: "CategoryEdit",
  setup() {

    const categories = ref([] as Category[]);
    const dialog = ref(false);
    const categoryName = ref("");
    const categoryDescription = ref("");

    async function loadTags() {
      categories.value = await fetchAllCategories();
    }

    loadTags();

    function openDialog() {
      dialog.value = true;
    }

    return {
      categories,
      loadTags,
      dialog,
      openDialog,
      categoryName,
      categoryDescription,
      deleteCategory,
      createCategory
    }

  }
})

</script>

<template>
  <v-card rounded="0">

    <v-toolbar color="#711a5f">
      <v-toolbar-title style="color: white;">NEO TODO - Categories</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn class="ma-2" @click="loadTags()" icon="mdi-sync" style="color: white;"></v-btn>
      <v-btn class="ma-2" @click="openDialog()" icon="mdi-plus" style="color: white;"></v-btn>
    </v-toolbar>

    <v-expansion-panels v-for="(category, _) in categories" :key="JSON.stringify(category)">
      <v-expansion-panel>
        <v-expansion-panel-title>
          <v-row>
            <v-col cols="2">
              <v-chip label color="#711a5f" style="width: 3em;">
                {{ category.id }}
              </v-chip>
            </v-col>
            <v-col cols="8">
              <div class="font-weight-bold text-h6">
                <v-icon color="#711a5f">mdi-shape-outline</v-icon>
                &nbsp;
                {{ category.name }}
              </div>
            </v-col>
          </v-row>
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <v-row>
            <v-col>
              <v-btn @click="async () => { await deleteCategory(category.id); await loadTags(); }"
                prepend-icon="mdi-delete" elevation="0">Delete</v-btn>
            </v-col>
          </v-row>
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-dialog v-model="dialog">
      <v-card>
        <v-card-title>
          Add a new category
        </v-card-title>
        <v-divider></v-divider>
        <v-card-text>
          <v-text-field v-model="categoryName" label="Name"></v-text-field>
        </v-card-text>
        <v-card-text>
          <v-text-field v-model="categoryDescription" label="Description"></v-text-field>
        </v-card-text>
        <v-card-actions class="justify-end">
          <v-card-action>
            <v-btn block @click="dialog = false">Cancel</v-btn>
          </v-card-action>
          <v-card-action>
            <v-btn block
              @click="async () => { await createCategory({ id: 0, name: categoryName, description: categoryDescription }); await loadTags(); dialog = false; }">Save</v-btn>
          </v-card-action>
        </v-card-actions>
      </v-card>
    </v-dialog>

  </v-card>
</template>