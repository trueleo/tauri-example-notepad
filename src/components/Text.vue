<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import { ref } from "vue";
import contenteditable from 'vue-contenteditable'
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event"

const text = ref("");

async function save() {
    let res = await invoke("save", { path: "hello.txt", buffer: text.value });
    console.log(res);
}

const unlisten = await listen("callback_save", async (event) => {
    await save()
})

</script>


<template>
    <div id="container">
        <contenteditable tag="div" id="input" v-model="text" />
        <div id="footer">
            <button @click="save">Save</button>
        </div>
    </div>
</template>

<style scoped>
* {
    max-width: none;
}

#container {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
}

#input {
    padding: 17px;
    flex-grow: 1;
    width: 100%;
    color: white;
    background-color: #101e1a;
}

#footer {
    color: white;
    flex-basis: 2rem;
    background-color: #126a51;
}

#footer>button {
    border-width: 0px;
    border-radius: 0.3rem;
    border-color: white;
    font-weight: 600;
    padding: 5px 10px;
    margin: 10px;
    margin-right: 18px;
    background-color: #093d2e;
    float: right;
    box-shadow: rgba(60, 64, 67, 0.3) 0px 1px 2px 0px, rgba(60, 64, 67, 0.15) 0px 2px 6px 2px;
}
</style>