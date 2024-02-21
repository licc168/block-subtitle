<script setup lang="ts">
import {appWindow} from '@tauri-apps/api/window';

import {listen} from '@tauri-apps/api/event'
import {Payload, KeyEventType} from '../types.ts';
import {ref} from "vue";

const isShow = ref(true);
await listen<Payload>('key', (event) => {
  const payLoad: Payload = event.payload;
  console.log(payLoad)
  if (payLoad.action === KeyEventType.Release ) {
    if( payLoad.key === "Num0") {
      if (isShow.value) {
        appWindow.hide();
        console.log("hide");
      } else {
        appWindow.show();
        console.log("show");

      }
      isShow.value = !isShow.value;
      content.value= "";

    }
    if( payLoad.key === "Equal") {
      changeOpacity(0.02);
      content.value= "";

    }
    if( payLoad.key === "Minus") {
      changeOpacity (-0.02);
      content.value= "";
    }

  }
})
const content = ref("快捷键 [0] 隐藏/显示.  [-]  减少透明度  [+] 新增透明图 ")
function changeOpacity(change: number) {
  const div = document.querySelector('.container') as HTMLElement;
  let currentOpacity = parseFloat(div.style.backgroundColor!.split(',')[3]);
  currentOpacity = isNaN(currentOpacity) ? 1 : currentOpacity;

  let newOpacity = currentOpacity + change;
  newOpacity = newOpacity < 0 ? 0 : newOpacity > 1 ? 1 : newOpacity;

  div.style.backgroundColor = `rgba(0, 0, 0, ${newOpacity})`;
}
</script>

<template>
  <div data-tauri-drag-region class="container"  >

  <h4  style="color: aqua" data-tauri-drag-region>{{ content }}</h4>
  </div>
</template>
