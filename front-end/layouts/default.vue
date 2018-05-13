<template>
  <div>
    <!-- <navbar /> -->
    <nuxt/>
  </div>
</template>
<script>
import Navbar from "~/components/Navbar.vue";

export default {
  components: {
    Navbar
  }
};
</script>

<style>
body,
html {
  margin: 0;
  padding: 0;
}
html {
  font-family: "Helvetica Neue", Helvetica, Arial, sans-serif;
  /* font-family: "Source Sans Pro", -apple-system, BlinkMacSystemFont, "Segoe UI",
    Roboto, "Helvetica Neue", Arial, sans-serif; */
}
</style>
