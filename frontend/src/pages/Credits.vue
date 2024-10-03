<script setup>
import Navbar from "@components/Navbar.vue";
</script>

<template>
    <Navbar selected="credits"/>
    <div class="container">
        <h1>Credits</h1>
        <p>
            Built by <a href="https://github.com/TheRedXD" target="_blank">@TheRedXD</a>, also known as TheRed!<br/>
            Massive thanks to <a href="https://github.com/TheBlueBurger" target="_blank">@TheBlueBurger</a> and <a href="https://github.com/JayTheCoderX" target="_blank">@JayTheCoderX</a><br/>for helping to build this project.<br/>
        </p>
    </div>
</template>

<style scoped lang="scss">
@import url(/fonts/IBM_Plex_Mono/font.css);

.container {
    display: block;
    background-color: #231D1B;
    padding: 20px;
    border-radius: 5px;
    border: 1px solid #3b312e;
    color: #ffc2ae;
    width: fit-content;
    margin: auto;
    margin-top: 50vh;
    transform: translateY(calc(-50% - 50px));
    h1 {
        color: #ffc2ae;
        font-size: 38px;
        font-family: "Roboto Light";
    }
    p {
        color: #ffd7c9;
        font-size: 18px;
        font-family: "IBM Plex Mono";
        margin-top: 5px;
    }
    a {
        color: #ffc2ae;
        text-decoration: none;
        transition: color 0.3s ease;
        &:hover {
            color: #9f8778;
        }
    }
}
</style>
