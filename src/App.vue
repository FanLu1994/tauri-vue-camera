<script setup>
import { useDevicesList } from '@vueuse/core'
import {onBeforeMount, onMounted, ref} from "vue";

const videoRef = ref(null)

onBeforeMount(async () => {
  await navigator.mediaDevices.getUserMedia({
    audio: true,
    video: true,
  });
})



const {
  devices,
  videoInputs: cameras,
  audioInputs: microphones,
  audioOutputs: speakers,
} = useDevicesList()

onMounted(()=>{

})

</script>

<template>
  <div class="container" data-tauri-drag-region>
    <video ref="videoRef" class="video"></video>

    <div class="settings">
      <div>
        <select>
          <option v-for="camera in cameras" :key="camera.deviceId" :value="camera.deviceId">
            {{ camera.label }}
          </option>
        </select>
      </div>

    </div>
  </div>
</template>

<style scoped>
</style>
