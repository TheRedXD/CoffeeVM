<script setup>
let props = defineProps({
    text: String,
    style: String,
    icon: String
});
</script>

<template>
    <div class="button" :style=props.style>
        <i v-if="props.icon" :class=props.icon></i>
        <p v-if="props.text">{{ props.text }}</p>
    </div>
</template>

<style scoped lang="scss">
    .button {
        --background: #33251F;
        --border: #382922;
        --background-hover: #3e2d26;
        --border-hover: #443129;
        --background-active: #33251F;
        --border-active: #443129;
        --text-color: #d1987f;
        --text-color-hover: #d1987f;
        --text-color-active: #d1987f;
        display: inline-block;
        background-color: var(--background);
        color: var(--text-color);
        cursor: pointer;
        width: fit-content;
        padding: 8px 12px;
        border-radius: 5px;
        user-select: none;
        border: 1px solid var(--border);
        transition: .1s all ease-in-out;
        &:hover {
            background-color: var(--background-hover);
            border-color: var(--border-hover);
            color: var(--text-color-hover);
        }
        &:active {
            background-color: var(--background-active);
            border-color: var(--border-active);
            color: var(--text-color-active);
            transition: none;
        }
        i, p {
            display: inline-block;
        }
        i {
            margin-right: 8px;
        }
    }
</style>
