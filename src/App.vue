<script setup>
import {onBeforeMount, onMounted, ref, watchEffect} from 'vue'
import { useDevicesList, useUserMedia } from '@vueuse/core'

onBeforeMount(async () => {
  await navigator.mediaDevices.getUserMedia({
    audio: true,
    video: true,
  });
})

const currentCamera = ref()
const videoRef = ref()
const { videoInputs: cameras } = useDevicesList({
  requestPermissions: true,
  onUpdated() {
    if (!cameras.value.find(i => i.deviceId === currentCamera.value)) {
      currentCamera.value = cameras.value[0].deviceId
      onCameraChange()
      }
    },
})

const { stream, enabled } = useUserMedia({
  constraints: { video: { deviceId: currentCamera } },
})

function onCameraChange() {
  console.log(currentCamera.value)
  console.log(cameras.value)
  if(currentCamera.value){
    let constraints = { video: { deviceId: currentCamera.value } }
    navigator.mediaDevices
        .getUserMedia({ video: constraints, audio: false })
        .then(function (stream) {
          videoRef.value.srcObject = stream;
          videoRef.value.play();
          // videoRef.value.addEventListener("canplay", onVideoCanPlay, false);
        });
  }
}

onMounted(()=>{
  if(cameras.value.length > 0){
    currentCamera.value = cameras.value[0].deviceId
    onCameraChange()
  }
})

const shape = ref('rectangle')
const onChangeShape = () => {
  shape.value = shape.value === 'rectangle' ? 'circle' : 'rectangle'
}

</script>

<template>
  <div class="container" data-tauri-drag-region>
<!--    <video ref="videoRef" class="video"></video>-->
    <video ref="videoRef" muted autoplay class="h-100 w-auto video"
           :class="shape==='circle'?'circle':'rectangle'"
           data-tauri-drag-region/>

    <div class="settings">
      <div @click="onChangeShape">
        切换形状
      </div>

      <div>
        <select v-model="currentCamera" @change="onCameraChange">
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
