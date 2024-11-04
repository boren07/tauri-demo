<template>
  <div class="m-ui-tiger">
    <div class="item">
      <ul class="roller">
        <li>
          <img src="../assets/images/tiger_awards_1.png" alt=""/>
        </li>
        <li>
          <img src="../assets/images/tiger_awards_2.png" alt=""/>
        </li>
        <li>
          <img src="../assets/images/tiger_awards_3.png" alt=""/>
        </li>
      </ul>
    </div>
    <div class="item">
      <ul class="roller">
        <li>
          <img src="../assets/images/tiger_awards_1.png" alt=""/>
        </li>
        <li>
          <img src="../assets/images/tiger_awards_2.png" alt=""/>
        </li>
        <li>
          <img src="../assets/images/tiger_awards_3.png" alt=""/>
        </li>
      </ul>
    </div>
    <div class="item">
      <ul class="roller">
        <li>
          <img src="../assets/images/tiger_awards_1.png" alt=""/>
        </li>
        <li>
          <img src="../assets/images/tiger_awards_2.png" alt=""/>
        </li>
        <li>
          <img src="../assets/images/tiger_awards_3.png" alt=""/>
        </li>
      </ul>
    </div>
    <a id="js_toggle" class="toggle" href="javascript:;" @click="()=>this.lottery.draw()"></a>
  </div>

</template>

<script>
import LotteryTiger from "../util/tiger.js";

export default {
  name: "Tiger",
  data() {
    return {
      lottery: undefined

    }
  },
  mounted() {
    this.init();
  },
  methods: {
    init() {
      this.lottery = new LotteryTiger(document.getElementById('js_toggle'), document.querySelectorAll('.roller')) // eslint-disable-line
      this.lottery.on('start', ()=> {
        setTimeout(()=> {
          var ret = [Math.round(Math.random() * 2), Math.round(Math.random() * 2), Math.round(Math.random() * 2)]
          this.lottery.setResult(ret)
//            lottery.reset();
        }, 1000)
      })
      this.lottery.on('end', ()=>  {

      })
    },


  }
}
</script>


<style scoped>

</style>