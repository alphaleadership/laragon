<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");
await invoke("executor",{cmde:"dir",arg:"/?"}).then((d)=>{
  let data=`${d}`
  let dataarray=[]
  console.log(typeof data)
  data=data.replace("[",'')
  data=data.replace("]",'')
  dataarray=data.split(",")
  console.log(dataarray)
  dataarray=dataarray.map((i)=>{
    console.log(String.fromCharCode(parseInt(i)))
    return String.fromCharCode(parseInt(i))
  })
  console.log(dataarray.join(""))
});
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
