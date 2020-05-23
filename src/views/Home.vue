<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png" />
    <!-- <HelloWorld msg="Welcome to Your Vue.js App" /> -->
    <button @click="woo">Do a Woo!</button>
  </div>
</template>

<script>
// @ is an alias to /src
// import HelloWorld from "@/components/HelloWorld.vue";

export default {
  name: "Home",
  // components: {
  //   HelloWorld
  // },
  methods: {
    woo() {
      console.log("woo");
      this.$wasm.greet();
    }
  }
};
</script>
