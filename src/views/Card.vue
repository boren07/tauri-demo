<template>
  <canvas class="m-ui-scratch" id="js_lottery"></canvas>
  <button class="m-ui-button" @click="()=>lottery.reset()">再来一次</button>
</template>
<script>
//刮刮卡
import LotteryCard from "../util/card.js";
import imgUrl from '../assets/images/placeholder.jpg'
import imgResultUrl from '../assets/images/scratch_no.png'
export default {
  name: "Card",
  data() {
    return {
      lottery: undefined
    }
  },
  mounted() {
    const img = new Image();
    img.src = imgUrl
    let that =  this;
    img.onload = function () {
      const lottery = new LotteryCard(document.getElementById('js_lottery'), { // eslint-disable-line
        cover: img
      });
      lottery.on('start', ()=>  {
        lottery.setResult(imgResultUrl)
      })
      lottery.on('end', ()=>  {

      })
      that.lottery = lottery
    }
  }
}
</script>


<style scoped>

</style>