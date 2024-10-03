<script setup>
import coffee from "@assets/coffee.png";
</script>

<template>
    <div class="nf">
        <div class="notfound">
            <img :src='coffee' />
            <div class="notfound-content">
                <h1>404</h1>
                <p>Oops! Looks like you spilled your coffee. Bummer. <a href="/">Perhaps we should make you another one?</a></p>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
// IBM Plex Mono
@import url("/fonts/IBM_Plex_Mono/font.css");

.notfound {
    text-align: center;
    height: 100vh;
    width: 60vw;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: row;
    // background-color: #f8f8f8;
    color: #faceb9;
    font-family: "IBM Plex Mono";
    font-weight: 300;
    padding: 20px;
    box-sizing: border-box;
    position: relative;
    .notfound-content {
        flex-direction: column;
        z-index: 10;
        animation-name: contentpopin;
        animation-duration: 1s;
        // animation-delay: 1s;
        animation-timing-function: cubic-bezier(.11,.7,.33,.98);
        animation-fill-mode: forwards;
        opacity: 0;
        @keyframes contentpopin {
            0% {
                transform: translateY(-50px);
                opacity: 0;
            }
            100% {
                transform: translateY(0px);
                opacity: 1;
            }
        }
    }

    &:before {
        content: "";
        position: absolute;
        width: 100vw;
        height: 100%;
        background-image: linear-gradient(to bottom right, #7e4444, #754a3d, #64412e);
        pointer-events: none;
        user-select: none;
        opacity: 0.8;
        z-index:-1;
    }
}

.nf {
    &:before {
        // vignette
        content: "";
        position: absolute;
        // top: 0;
        // left: 0;
        width: 100vw;
        height: 100vh;
        background-image: radial-gradient(circle, rgba(0, 0, 0, 0.4) 0%, rgba(0, 0, 0, 0.1) 20%, rgba(0, 0, 0, 0.0) 40%);
        background-position-x: -20vw;
        pointer-events: none;
        user-select: none;
        opacity: 1.0;
        z-index: 1;
    }
}

img {
    width: 200px;
    margin: 0 auto;
    display: block;
    z-index: 10;
    animation-name: coffeepopin;
    animation-duration: 2s;
    animation-delay: 0.3s;
    animation-timing-function: cubic-bezier(.11,.7,.33,.98);
    animation-fill-mode: forwards;
    opacity: 0;
    @keyframes coffeepopin {
        0% {
            transform: scale(0.5);
            opacity: 0;
        }
        100% {
            transform: scale(1.0);
            opacity: 1;
        }
    }
}

h1 {
    text-align: center;
    font-size: 128px;
    margin-bottom: 20px;
    font-family: "IBM Plex Mono";
    font-weight: 100;
}

p {
    text-align: center;
    font-size: 24px;
    margin-bottom: 40px;
    max-width: 600px;
}

a {
    color: #faceb9;
    text-decoration: underline;
    transition: color 0.1s ease;

    &:hover {
        color: #ffffff;
    }
}
</style>
