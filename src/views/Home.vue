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
    async woo() {
      console.log("woo");
      console.log(this.$wasm);

      const client = await this.$wasm.start_client(
        "/ip4/127.0.0.1/tcp/61883/ws",
        "info"
      );
      console.log("rust libp2p client:", client);
    }
  }
};
</script>
