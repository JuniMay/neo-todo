<script lang="ts">

import { defineComponent, ref } from 'vue';
import { Tag } from '../utils';

import { fetchAllTags, createTag, deleteTag } from '../utils';

export default defineComponent({
  name: "TagEdit",
  setup() {

    const tags = ref([] as Tag[]);
    const dialog = ref(false);
    const tagName = ref("");

    async function loadTags() {
      tags.value = await fetchAllTags();
    }

    loadTags();

    function openDialog() {
      dialog.value = true;
    }

    return {
      tags,
      loadTags,
      dialog,
      openDialog,
      tagName,
      createTag,
      deleteTag,
    }

  }
})

</script>

<template>
  <v-card rounded="0">

    <v-toolbar color="#711a5f">
      <v-toolbar-title style="color: white;">NEO TODO</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn class="ma-2" @click="loadTags()" icon="mdi-sync" style="color: white;"></v-btn>
      <v-btn class="ma-2" @click="openDialog()" icon="mdi-plus" style="color: white;"></v-btn>
    </v-toolbar>

    <v-expansion-panels v-for="(tag, _) in tags" :key="JSON.stringify(tag)">
      <v-expansion-panel>
        <v-expansion-panel-title>
          <v-row>
            <v-col cols="2">
              <v-chip label color="#711a5f" style="width: 3em;">
                {{ tag.id }}
              </v-chip>
            </v-col>
            <v-col cols="8">
              <div class="font-weight-bold text-h6">
                <v-icon color="#711a5f">mdi-tag-outline</v-icon>
                &nbsp;
                {{ tag.name }}
              </div>
            </v-col>
          </v-row>
        </v-expansion-panel-title>
        <v-expansion-panel-text>
          <v-row>
            <v-col>
              <v-btn @click="async () => { await deleteTag(tag.id); await loadTags(); }" prepend-icon="mdi-delete"
                elevation="0">Delete</v-btn>
            </v-col>
          </v-row>
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <v-dialog v-model="dialog">
      <v-card>
        <v-card-title>
          Add a new tag
        </v-card-title>
        <v-divider></v-divider>
        <v-card-text>
          <v-text-field v-model="tagName"></v-text-field>
        </v-card-text>
        <v-card-actions class="justify-end">
          <v-card-action>
            <v-btn block @click="dialog = false">Cancel</v-btn>
          </v-card-action>
          <v-card-action>
            <v-btn block
              @click="async () => { await createTag({ id: 0, name: tagName }); await loadTags(); dialog = false; }">Save</v-btn>
          </v-card-action>
        </v-card-actions>
      </v-card>
    </v-dialog>

  </v-card>
</template>