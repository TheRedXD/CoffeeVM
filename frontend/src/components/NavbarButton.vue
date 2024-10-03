<script setup>
let props = defineProps({
    text: String,
    style: String,
    icon: String,
    selected: Boolean
});
</script>

<template>
    <div :class="'button' + (props.selected ? ' selected' : '')" :style=props.style>
        <div class="content">
            <i v-if="props.icon" :class=props.icon></i>
            <p v-if="props.text">{{ props.text }}</p>
        </div>
    </div>
</template>

<style scoped lang="scss">
    .button {
        --background: #231d1b;
        --border: #231d1b;
        --background-hover: #3e2d26;
        --border-hover: #443129;
        --background-active: #33251F;
        --border-active: #443129;
        --text-color: #d1987f;
        --text-color-hover: #d1987f;
        --text-color-active: #d1987f;
        height: 50px;
        font-size: 16px;
        display: inline-flex;
        vertical-align: middle;
        justify-content: center;
        flex-direction: column;
        background-color: var(--background);
        color: var(--text-color);
        cursor: pointer;
        width: fit-content;
        padding: 0px 14px;
        border-radius: 0px;
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
            display: inline-flex;
        }
        i {
            margin-right: 8px;
        }
        p {
            position: relative;
            vertical-align: middle;
        }
        .content {
            flex-direction: row;
        }
        &.selected {
            border-bottom: 1px solid #d1987f;
            background-color: #33251F;
            .content {
                transform: translateY(0px);
                animation-name: selectedpop;
                animation-duration: 0.2s;
                animation-timing-function: cubic-bezier(.11,.7,.33,.98);
                animation-fill-mode: forwards;
                @keyframes selectedpop {
                    0% {
                        transform: translateY(0px);
                    }
                    50% {
                        transform: translateY(5px);
                    }
                    100% {
                        transform: translateY(0px);
                    }
                }
            }
        }
    }

    @media screen and (max-width: 1000px) {
        .button {
            p {
                display: none;
            }
            i {
                margin-right: 0px;
            }
        }
    }
</style>
