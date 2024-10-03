<script setup>
    import { computed, onMounted, onUnmounted, ref } from "vue";
    
    const canvasElem = ref();
    
    let ctx = computed(() => {
        return canvasElem.value.getContext("2d");
    });

    import * as chart from "chart.js/auto";

    let leChart = ref();

    onMounted(() => {
        leChart.value = new chart.Chart(ctx.value, {
            type: "line",
            data: {
                datasets: [{
                    label: "crab",
                    data: [1, 2, 3],
                    pointHitRadius: 20
                }],
                labels: [1, 2, 3],
            },
        });
    });
</script>

<template>
    <div class="chart-element">
        <canvas class="chart" ref="canvasElem" />
    </div>
</template>
