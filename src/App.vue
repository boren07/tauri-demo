<template>
  <main class="container">
    <div>
      <h5>欢迎您：<span style="color: #0e9bea">{{username}}</span></h5>
      <div class="radio-input">
        <label class="label">
          <div class="back-side"></div>
          <input type="radio" id="value-1" v-model="checkType" name="value-radio" :value="1" />
          <span class="text">大转盘</span>
          <span class="bottom-line"></span>
        </label>

        <label class="label">
          <div class="back-side"></div>
          <input type="radio" id="value-2" v-model="checkType"  name="value-radio" :value="2" />
          <span class="text">老虎机</span>
          <span class="bottom-line"></span>
        </label>

        <label class="label">
          <div class="back-side"></div>
          <input type="radio" id="value-3" v-model="checkType"  name="value-radio" :value="3" />
          <span class="text">刮刮卡</span>
          <span class="bottom-line"></span>
        </label>
      </div>
    </div>
    <div>
      <Dial v-if="checkType ==1 "/>
      <Tiger v-else-if="checkType==2 "/>
      <Card v-else-if="checkType==3 "/>

    </div>

  </main>

</template>
<script>
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import Tiger from "./views/Tiger.vue";
import Card from "./views/Card.vue";
import Dial from "./views/Dial.vue";
const greetMsg = ref("");
const name = ref("");

/*调用rust函数*/
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", {name: name.value});
}

export default {
  components: {
    Tiger,Card,Dial
  },
  data() {
    return {
      prizes: [
        '一等奖(Iphone 16 Pro Max)', '二等奖（Switch游戏机）', '三等奖（2000元）', '四等奖（1000元）',
        '五等奖（800元）', '六等奖（500元）', '七等奖（300元）', '八等奖（100元）'
      ],
      lastPrizeTime: new Date(),
      checkType: 1,
      username: undefined,
    }
  },
  computed:{

  },
  created() {

  },
  mounted() {
    this.rsGetUserName();
  },
  methods: {
    handleClick() {
      const wheel = document.getElementById('wheel');
      const randomIndex = Math.floor(Math.random() * this.prizes.length);
      const targetAngle = (randomIndex * 45) + (360 * 3); // 每个奖品区域45度，旋转3圈
      wheel.style.transform = `rotate(${targetAngle}deg)`;
      setTimeout(() => {
        alert(`恭喜你，获得：<b>${this.prizes[randomIndex]}</b>`);
      }, 3000); // 动画时间
    },
    async rsGetUserName() {
      let osName = await invoke('get_os_username')
      this.username = osName
    }
  }
}

</script>

<style scoped>
.container{

}
/* From Uiverse.io by m1her */
.radio-input {
  display: flex;
  align-items: center;
  gap: 6px;
  //background-color: none;
  padding: 6px;
  border-radius: 8px;
  overflow: hidden;
  height: 94px;
}

.radio-input input {
  display: none;
}

.radio-input .label {
  width: 70px;
  height: 80px;
  background-color: #2a2a2a;
  border-radius: 4px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  padding: 8px 6px;
  border-top: 1px solid #383838;
  transition: all 0.1s linear;
  position: relative;
  z-index: 2;
}

.label .back-side {
  position: absolute;
  top: -10px;
  left: 0px;
  background-color: #2a2a2a;
  border-radius: 4px 4px 2px 2px;
  width: 100%;
  height: 14px;
  box-shadow:
      inset 0 5px 3px 1px rgba(0, 0, 0, 0.5),
      inset 0px -5px 2px 0px rgba(56, 163, 224, 0.1);
  transform: perspective(300px) rotateX(50deg);
  z-index: 1;
  opacity: 0;
  transition: all 0.1s linear;
}

.label:has(input[type="radio"]:checked) .back-side {
  opacity: 1;
}

.label:has(input[type="radio"]:checked) {
  transform: perspective(200px) rotateX(-18deg);
  transform-origin: 50% 40%;
  box-shadow: inset 0px -20px 15px 0px rgba(0, 0, 0, 0.5);
  border-top: 1px solid #2589c362;
  margin-top: 6px;
  border-radius: 0 0 4px 4px;
}

.label .text {
  color: #ffffff;
  font-size: 15px;
  line-height: 12px;
  padding: 0px;
  font-weight: 800;
  text-transform: uppercase;
  transition: all 0.1s linear;
  text-shadow: -1px -1px 1px rgb(224, 224, 224, 0.1);
}

.label input[type="radio"]:checked + .text {
  color: #0e9bea;
  text-shadow:
      0px 0px 8px rgb(37, 138, 195),
      1px 1px 2px rgb(0, 0, 0, 1);
}

.label .bottom-line {
  width: 100%;
  height: 4px;
  border-radius: 999px;
  background-color: #2a2a2a;
  box-shadow: 0 0 3px 0px rgb(19, 19, 19);
  border-top: 1px solid #383838;
  transition: all 0.1s linear;
}

.label:has(input[type="radio"]:checked) .bottom-line {
  background-color: #1a1a1a;
  border-top: 1px solid #258ac340;
}

</style>