<script setup>
import {onBeforeMount, onMounted, ref, watchEffect} from 'vue'
import { useDevicesList, useUserMedia } from '@vueuse/core'
import HoverSelect from "./components/HoverSelect.vue";
import {CameraOne,Round,RectangleOne,FlipHorizontally,Power} from "@icon-park/vue-next";


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
      onCameraChange(currentCamera.value)
      }
    },
})

const { stream, enabled } = useUserMedia({
  constraints: { video: { deviceId: currentCamera } },
})

let currentStream;
function onCameraChange(cameraId) {
  currentCamera.value = cameraId
  if (currentStream){
    currentStream.getTracks().forEach(track => track.stop());
  }
  if(currentCamera.value){
    let constraints = { deviceId: { exact: currentCamera.value } };
    navigator.mediaDevices
        .getUserMedia({ video: constraints, audio: false })
        .then(function (stream) {
          videoRef.value.srcObject = stream;
          currentStream = stream;
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

const mirror = ref(true)

// 关闭程序
const onClose = () => {

}

</script>

<template>
  <div class="container" data-tauri-drag-region>
    <power theme="outline" size="24" fill="#ef4444" class="close-btn"/>

<!--    <video ref="videoRef" class="video"></video>-->
    <video ref="videoRef" muted autoplay class="h-100 w-auto video"
           :class="{'circle':shape==='circle','rectangle':shape==='rectangle','mirror':mirror?'mirror':''}"
           data-tauri-drag-region/>

    <div class="settings">
      <div @click="onChangeShape" title="形状切换">
        <round v-show="shape==='rectangle'" theme="outline" size="24" fill="#ea580c"/>
        <rectangle-one v-show="shape==='circle'" theme="outline" size="24" fill="#ea580c"/>
      </div>

      <div>
<!--        <select v-model="currentCamera" @change="onCameraChange">-->
<!--          <option v-for="camera in cameras" :key="camera.deviceId" :value="camera.deviceId">-->
<!--            {{ camera.label }}-->
<!--          </option>-->
<!--        </select>-->
        <hover-select :options="cameras" @change="onCameraChange" label="label" value="deviceId" >
          <template #icon>
            <camera-one theme="outline" size="24" fill="blue"/>
          </template>
        </hover-select>
      </div>

      <div @click="mirror=!mirror" title="镜像">
        <flip-horizontally theme="outline" size="24" fill="#9333ea"/>
      </div>


    </div>
  </div>
</template>

<style scoped>
</style>
