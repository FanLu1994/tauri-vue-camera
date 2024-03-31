<script setup>

const props = defineProps({
  options: {
    type: Array,
    default: () => []
  },
  label: {
    type: String,
    default: 'label'
  },
  value: {
    type: String,
    default: ''
  }
})

const emit = defineEmits(['change'])

</script>

<template>
  <div class="hover-select">
    <div class="hover-icon">
      <slot name="icon">
        please select
      </slot>
    </div>
    <div class="hover-items">
      <div class="hover-item" v-for="option in options"
           :title="option[label]"
           :key="option" @click="emit('change', option[value])">
        {{ option[label] }}
      </div>
    </div>
  </div>
</template>

<style scoped>

.hover-select{
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  position: relative;
  cursor: pointer;
  z-index: 999;
}

.hover-items{
  display: none;
  position: absolute;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
  width: 200px;
  border-radius: 10px;
  padding: 2px 4px;
  background-color: rgba(255,255,255,0.3);
  backdrop-filter: blur(10px);
}

.hover-item{
  display: flex;
  justify-content: flex-start;
  white-space: nowrap; /* 禁止换行 */
  overflow: hidden; /* 隐藏溢出的文本 */
  text-overflow: ellipsis; /* 使用省略号代替溢出的文本 */
  border-radius: 5px;
}

.hover-item:hover{
  background-color: rgba(255,255,255,0.5);
}

.hover-icon {
  display: flex;
  align-items: center;
  justify-content: center;
}

.hover-select:hover .hover-items{
  display: block;
}

</style>