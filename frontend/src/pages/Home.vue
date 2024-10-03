<script setup>
import Navbar from "@components/Navbar.vue";
import Button from "@components/Button.vue";
import CPUChart from "@components/CPUChart.vue";

import { useCounterStore } from "@stores/counter.js";

let counter = useCounterStore();
counter.increment();
</script>

<template>
    <Navbar selected="dashboard"/>
    <div class="home-content">
        <CPUChart />
        <!-- {{ counter.count }} -->
        <!-- <Button text="chungus" icon="fa-solid fa-cube" /> -->
    </div>
</template>

<style scoped lang="scss"></style>
