<script setup>
import { ref } from "vue";
let props = defineProps({
    username: String,
    style: String,
    icon: String
});

let opened = ref(false);
let animOpened = ref(false);
</script>

<template>
    <div class="widget">
        <div class="button" :style=props.style @click="opened = !opened; if(opened) {animOpened = true} else {setTimeout(() => {animOpened = false}, 100)}">
            <div class="content">
                <i class="user fa-solid fa-user"></i>
                <p v-if="props.username">{{ props.username }}</p>
                <i :class="'caret fa-solid' + (opened.valueOf() ? ' fa-caret-up' : ' fa-caret-down')"></i>
            </div>
        </div>
        <div :class="'dropdown' + (opened.valueOf() ? '' : ' close')" v-if="animOpened.valueOf()">
            <p>chungus</p>
        </div>
    </div>
</template>

<style scoped lang="scss">
    .widget {
        height: 50px;
        display: inline-flex;
        flex-direction: column;
        vertical-align: middle;
        justify-content: center;
        .dropdown {
            position: absolute;
            top: 55px;
            width: 250px;
            right: 0px;
            padding: 10px 20px;
            background-color: #2f2724;
            border: 1px solid #4d403b;
            box-shadow: 0px 0px 10px 0px rgba(0, 0, 0, 0.4);
            border-radius: 5px;
            z-index: 10;
            animation-name: dropdownin;
            animation-duration: 0.1s;
            animation-timing-function: ease-in-out;
            animation-fill-mode: forwards;
            @keyframes dropdownin {
                0% {
                    opacity: 0;
                    transform: translateY(-20px);
                }
                100% {
                    opacity: 1;
                    transform: translateY(0);
                }
            }
            &.close {
                animation-name: dropdownout;
                animation-duration: 0.1s;
                animation-timing-function: ease-in-out;
                animation-fill-mode: forwards;
                pointer-events: none;
                user-select: none;
                @keyframes dropdownout {
                    0% {
                        opacity: 1;
                        transform: translateY(0);
                    }
                    100% {
                        opacity: 0;
                        transform: translateY(-20px);
                    }
                }
            }
        }
    }
    .button {
        height: 40px;
        font-size: 16px;
        --background: #2f2724;
        --border: #2f2724;
        --background-hover: #3e2d26;
        --border-hover: #443129;
        --background-active: #33251F;
        --border-active: #443129;
        --text-color: #d1987f;
        --text-color-hover: #d1987f;
        --text-color-active: #d1987f;
        display: inline-flex;
        vertical-align: middle;
        justify-content: center;
        flex-direction: column;
        background-color: var(--background);
        color: var(--text-color);
        cursor: pointer;
        width: fit-content;
        padding: 0px 10px;
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
        i.user {
            margin-right: 8px;
        }
        i.caret {
            margin-left: 10px;
        }
        p {
            display: inline-flex;
            position: relative;
            vertical-align: middle;
        }
        .content {
            flex-direction: row;
        }
    }
</style>