<script setup>
import {onBeforeMount, onMounted, ref, watchEffect} from 'vue'
import { useDevicesList, useUserMedia } from '@vueuse/core'
import HoverSelect from "./components/HoverSelect.vue";
import {CameraOne,Round,RectangleOne,FlipHorizontally,Power,Copy} from "@icon-park/vue-next";
import { invoke } from '@tauri-apps/api/tauri'
import {WebviewWindow} from "@tauri-apps/api/window";


const second = ref("")
onMounted(()=>{
  // 获取当前页面的 URL
  const url = window.location.href;
  console.log(url)
  // 创建 URLSearchParams 对象，用于处理 URL 查询参数
  const params = new URLSearchParams(new URL(url).search);
  // 获取特定参数的值
  const paramValue = params.get('second');
  console.log('参数值:', paramValue);
  if(paramValue){
    second.value = paramValue
  }
})

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
const onClose = async () => {
  await invoke('close', {})
}

// 打开新窗口
let win;
const onOpen = async () => {
  if (win){
    win.close()
  }
  win = new WebviewWindow("Second",
      {
        label: 'Second',
        title: '副窗口',
        url: '/?second=1',
        width: 300,
        height: 200,
        resizable: true,
        alwaysOnTop: false,
        transparent:true,
        hiddenTitle:true,
        decorations:false,
      }
  )

  // 窗口创建完毕/失败
  win.once('tauri://created', async() => {
    console.log('window create success!')
  })

  win.once('tauri://error', async() => {
    console.log('window create error!')
  })
}

const closeWin = async()=>{
}

</script>

<template>
  <div class="container" data-tauri-drag-region>
    <power theme="outline" size="24" fill="#ef4444" class="close-btn"
           v-show="second===''"
           @click="onClose"/>
<!--    <video ref="videoRef" class="video"></video>-->
    <video ref="videoRef" muted autoplay class="h-100 w-auto video"
           :class="{'circle':shape==='circle','rectangle':shape==='rectangle','mirror':mirror?'mirror':''}"
           data-tauri-drag-region/>

    <div class="settings" :style="{background:shape==='circle'?'transparent':''}">
      <div @click="onChangeShape" title="形状切换">
        <round v-show="shape==='rectangle'" theme="outline" size="24" fill="#051e28"/>
        <rectangle-one v-show="shape==='circle'" theme="outline" size="24" fill="#002b39"/>
      </div>

      <div>
<!--        <select v-model="currentCamera" @change="onCameraChange">-->
<!--          <option v-for="camera in cameras" :key="camera.deviceId" :value="camera.deviceId">-->
<!--            {{ camera.label }}-->
<!--          </option>-->
<!--        </select>-->
        <hover-select :options="cameras" @change="onCameraChange" label="label" value="deviceId" >
          <template #icon>
            <camera-one theme="outline" size="24" fill="#004650"/>
          </template>
        </hover-select>
      </div>

      <div @click="mirror=!mirror" title="镜像">
        <flip-horizontally theme="outline" size="24" fill="#005b55"/>
      </div>

      <div @click="onOpen" title="新开窗口" v-show="second===''">
        <copy theme="outline" size="24" fill="#005b55"/>
      </div>

    </div>
  </div>
</template>

<style scoped>
</style>
