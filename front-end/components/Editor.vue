<template>
  <codemirror ref="codeMirrorComponent" :options="{ lineNumbers: true }" class="code-window" />
</template>

<script>
let codemirror;
if (process.browser) {
  codemirror = require("vue-codemirror").codemirror;
}

export default {
  mounted() {
    this.$refs.codeMirrorComponent.codemirror.setSize("100%", "100%");
  },
  components: {
    codemirror
  }
};
</script>
<style lang="scss" scoped>
.code-window {
  position: relative;
  margin-right: auto;
  margin-left: auto;
  width: 100%;
  @media (min-width: 768px) {
    width: 750px;
  }
  @media (min-width: 992px) {
    width: 970px;
  }
  height: 480px;
  border-radius: 4px;
  background: #fff;
  margin: auto;
  box-shadow: 0 4px 7px rgba(0, 0, 0, 0.2);
  overflow: hidden;
  margin-bottom: 1.5rem;
}
</style>
<style>
@import "codemirror/lib/codemirror.css";
</style>
