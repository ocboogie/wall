<template>
  <div class="container">
    <div class="title">New Wall</div>
    <editor/>
    <el-button type="primary" plain class="submit-button">Submit</el-button>
  </div>
</template>
<script>
import Editor from "./Editor.vue";

export default {
  components: {
    Editor
  }
};
</script>

<style lang="scss" scoped>
.container {
  display: flex;
  flex-direction: column;
  z-index: 1;
  .title {
    font-size: 3rem;
    color: #fff;
    margin-bottom: 2rem;
    text-align: center;
  }
  .submit-button {
    margin: auto;
    margin-bottom: 1.5rem;
    box-shadow: 0 2px 7px rgba(0, 0, 0, 0.2);
  }
}
</style>
<style>
@import "codemirror/lib/codemirror.css";
</style>

