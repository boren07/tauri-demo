<template>
  <div class="m-ui-dial">
    <div id="js_pointer" class="pointer">
      <a class="btn" href="javascript:;" @click="()=>this.lottery.draw()"></a>
    </div>
  </div>
</template>

<script>
//大转盘
import LotteryDial from "../util/dial.js";
import {sendNotice} from "../util/notice.js";
export default {
  name: "Dial",
  data() {
    return {
      lottery: undefined,
      count: 1,
      gift: ['笔记本📓','谢谢惠顾😯','SONY索尼 ZV-E10','谢谢惠顾🙂','ThinkPad联想笔记本','谢谢惠顾😅','iPhone16 Pro Max','谢谢惠顾i😫']
    }
  },
  mounted() {
    this.lottery = new LotteryDial(document.getElementById('js_pointer'), { // eslint-disable-line
      speed: 30, // 每帧速度
      areaNumber: 8 // 奖区数量
    })
    var index = -1
    this.lottery.on('start', ()=> {
      // 请求获取中奖结果
      index = Math.round(Math.random() * 7)
      this.lottery.setResult(index)
//    // 假如请求出错
//    setTimeout(function () {
//      lottery.reset()
//    }, 1000)
    })
    this.lottery.on('end',  ()=> {
      sendNotice(`恭喜您抽中了：${this.gift[index]}`)
      alert(`恭喜您抽中了：${this.gift[index]}`)
    })
  }
}
</script>



<style scoped>

</style>