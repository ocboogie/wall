<template>
  <div class="container">
    <div class="title">New Wall</div>
    <div class="code-window">
      <div class="code-editor">// Feel free to play with the code. :) function Hey(phrase) { phrase = phrase || "Hey"; return console.log(phrase); } window.onLoad = function() { Hey(); Hey("Jude"); Hey("Don't make it bad."); }
      </div>
    </div>
  </div>
</template>
<script>
import "prismjs";
import "prismjs/plugins/line-numbers/prism-line-numbers";
import CodeFlask from "codeflask";

export default {
  mounted() {
    var flask = new CodeFlask();
    // flask.run(".code-editor");
    flask.run(".code-editor", { lineNumbers: true });
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
  .code-window {
    width: 700px;
    height: 480px;
    border-radius: 4px;
    background: #fff;
    margin: auto;
    box-shadow: 0 4px 7px rgba(0, 0, 0, 0.2);
    overflow: hidden;
    .code-editor {
      position: relative;
      height: 480px;
      color: #444;
    }
  }
}
</style>
<style>
@import "prismjs/themes/prism.css";
@import "prismjs/plugins/line-numbers/prism-line-numbers.css";
@import "codeflask/src/codeflask.css";
</style>

