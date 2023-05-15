<script lang="ts">


import { defineComponent, ref } from 'vue';
import { taskKind, taskIcon } from '../utils';
import Common from './TaskEdit/Common.vue';
import Duration from './TaskEdit/Duration.vue'
import Reminder from './TaskEdit/Reminder.vue'

export default defineComponent({
    name: "TaskEdit",
    components: {
        Common,
        Duration,
        Reminder
    },
    setup(props) {
        const task = props.task;
        const kind = ref(props.task.kind);

        return {
            task,
            kind
        }
    },
    props: {
        task: {
            type: Object,
            required: true
        }
    }
})

</script>

<template>
    <template v-if="kind === 0">
        <Common :task="task"></Common>
    </template>
    <template v-else-if="kind === 1">
        <Duration :task="task"></Duration>
    </template>
    <template v-else>
        <Reminder :task="task"></Reminder>
    </template>
</template>