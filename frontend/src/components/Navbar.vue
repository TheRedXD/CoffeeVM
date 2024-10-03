<script setup>
import NavbarButton from "@components/NavbarButton.vue";
import AccountWidget from "@components/AccountWidget.vue";

import { RouterLink } from "vue-router";

let props = defineProps({
    selected: String
});
</script>

<template>
    <div class="navbar">
        <div class="content">
            <img src="https://fs.infra.thered.sh/branding/coffeevm/tag_nobg.png"/>
            <div class="buttons">
                <RouterLink to="/"><NavbarButton text="Dashboard" icon="fa-solid fa-fire" :selected="props.selected == 'dashboard' ? true : false"/></RouterLink>
                <RouterLink to="/containers"><NavbarButton text="Containers" icon="fa-solid fa-cube" :selected="props.selected == 'containers' ? true : false"/></RouterLink>
                <RouterLink to="/volumes"><NavbarButton text="Volumes" icon="fa-solid fa-jar" :selected="props.selected == 'volumes' ? true : false"/></RouterLink>
                <RouterLink to="/terminal"><NavbarButton text="Terminal" icon="fa-solid fa-terminal" :selected="props.selected == 'terminal' ? true : false"/></RouterLink>
                <RouterLink to="/modules"><NavbarButton text="Modules" icon="fa-solid fa-microchip" :selected="props.selected == 'modules' ? true : false"/></RouterLink>
                <RouterLink to="/credits"><NavbarButton text="Credits" icon="fa-solid fa-scroll" :selected="props.selected == 'credits' ? true : false"/></RouterLink>
            </div>
            <div class="info">
                <AccountWidget username="placebonker"/>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
    .content {
        width: 100%;
        height: 50px;
    }
    .navbar {
        width: 100%;
        height: 50px;
        background-color: #231d1b;
        display: block;
        img {
            display: inline-flex;
            position: relative;
            vertical-align: middle;
            height: 50px;
            margin-left: 0px;
            float: left;
            user-select: none;
        }
        .buttons {
            display: inline-flex;
            position: relative;
            vertical-align: middle;
            height: 50px;
            margin: auto;
            margin-right: 0px;
            width: fit-content;
            float: left;
            user-select: none;
        }
        .info {
            display: inline-flex;
            position: relative;
            vertical-align: middle;
            justify-content: center;
            flex-direction: row;
            height: 50px;
            margin: auto;
            margin-right: 5px;
            float: right;
        }
    }
</style>
