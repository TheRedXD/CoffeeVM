<script setup>
import Navbar from "@components/Navbar.vue";
import { buildAPIUrl } from "../util/apiUrl.js";
import { ref } from "vue";

let containers = ref([]);

async function fetchContainers() {
    let c = (await (await fetch(buildAPIUrl("/containers"))).json()).containers;
    containers.value = c;
}

fetchContainers();
</script>

<template>
    <Navbar selected="containers"/>
    <div class="containers-content">
        <div class="containers">
            <div v-for="container in containers" class="container">
                <i class="fa-solid fa-cube"></i>
                <p>{{ container.name }}</p>
                <p>Docker name: {{ container.dockerName }}</p>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
.containers-content .containers .container {
    background-color: #231D1B;
    border-radius: 10px;
    padding: 20px;
    margin: 20px;
    p, i {
        display: inline-block;
        color: #d1987f;
        font-size: 20px;
    }
    p:nth-of-type(2) {
        font-size: 16px;
        margin-left: 10px;
        padding-left: 10px;
        border-left: 1px solid #d1987f;
    }
    i {
        margin-right: 10px;
    }
}
</style>
