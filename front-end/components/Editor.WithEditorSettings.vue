<template>
  <div class="code-window" id="code-editor">
    <div class="dropdown">
      <font-awesome-icon class="settings-cog" @click="toggleSettings" :icon="showSettings ? faTimes : faCog" />
      <div class="dropdown-content" :class="{show: showSettings}">
        <editor-settings @theme-change="changeTheme" />
      </div>
    </div>
  </div>
</template>

<script>
import themes from "~/assets/themes.json";
import EditorSettings from "./EditorSettings.vue";

import FontAwesomeIcon from "@fortawesome/vue-fontawesome";
import faCog from "@fortawesome/fontawesome-free-solid/faCog";
import faTimes from "@fortawesome/fontawesome-free-solid/faTimes";

let CodeMirror;
if (process.browser) {
  CodeMirror = require("codemirror");
  // require("codemirror/mode/meta");
  // console.log(CodeMirror.modeInfo);
  // require("codemirror/mode/javascript/javascript.js");
}

export default {
  mounted() {
    this.codeMirror = CodeMirror(document.getElementById("code-editor"), {
      lineNumbers: true,
      mode: "javascript"
    });
    this.codeMirror.setSize("100%", "100%");
  },
  data: () => ({
    faTimes,
    faCog,
    currentTheme: null,
    codeMirror: null,
    showSettings: false
  }),
  methods: {
    toggleSettings() {
      this.showSettings = !this.showSettings;
    },
    changeTheme(value) {
      this.codeMirror.setOption("theme", value);
    }
  },
  components: {
    FontAwesomeIcon,
    EditorSettings
  }
  // head() {
  //   return {
  //     link: themes.map(themeName => ({
  //       rel: "stylesheet",
  //       type: "text/css",
  //       href: `https://codemirror.net/theme/${themeName}.css`
  //     }))
  //   };
  // }
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
  .dropdown {
    right: 0;
    position: absolute;
    z-index: 4;
    margin-top: 15px;
    margin-right: 20px;
    .settings-cog {
      width: 1.5em;
      height: 1.5em;
      cursor: pointer;
    }
    .dropdown-content {
      position: absolute;
      right: 0;
      display: none;
      &.show {
        display: block;
      }
    }
  }
}
</style>
<style>
@import "codemirror/lib/codemirror.css";

@import "codemirror/theme/3024-day.css";
@import "codemirror/theme/3024-night.css";
@import "codemirror/theme/abcdef.css";
@import "codemirror/theme/ambiance-mobile.css";
@import "codemirror/theme/ambiance.css";
@import "codemirror/theme/base16-dark.css";
@import "codemirror/theme/base16-light.css";
@import "codemirror/theme/bespin.css";
@import "codemirror/theme/blackboard.css";
@import "codemirror/theme/cobalt.css";
@import "codemirror/theme/colorforth.css";
@import "codemirror/theme/dracula.css";
@import "codemirror/theme/duotone-dark.css";
@import "codemirror/theme/duotone-light.css";
@import "codemirror/theme/eclipse.css";
@import "codemirror/theme/elegant.css";
@import "codemirror/theme/erlang-dark.css";
@import "codemirror/theme/gruvbox-dark.css";
@import "codemirror/theme/hopscotch.css";
@import "codemirror/theme/icecoder.css";
@import "codemirror/theme/idea.css";
@import "codemirror/theme/isotope.css";
@import "codemirror/theme/lesser-dark.css";
@import "codemirror/theme/liquibyte.css";
@import "codemirror/theme/lucario.css";
@import "codemirror/theme/material.css";
@import "codemirror/theme/mbo.css";
@import "codemirror/theme/mdn-like.css";
@import "codemirror/theme/midnight.css";
@import "codemirror/theme/monokai.css";
@import "codemirror/theme/neat.css";
@import "codemirror/theme/neo.css";
@import "codemirror/theme/night.css";
@import "codemirror/theme/oceanic-next.css";
@import "codemirror/theme/panda-syntax.css";
@import "codemirror/theme/paraiso-dark.css";
@import "codemirror/theme/paraiso-light.css";
@import "codemirror/theme/pastel-on-dark.css";
@import "codemirror/theme/railscasts.css";
@import "codemirror/theme/rubyblue.css";
@import "codemirror/theme/seti.css";
@import "codemirror/theme/shadowfox.css";
@import "codemirror/theme/solarized.css";
@import "codemirror/theme/ssms.css";
@import "codemirror/theme/the-matrix.css";
@import "codemirror/theme/tomorrow-night-bright.css";
@import "codemirror/theme/tomorrow-night-eighties.css";
@import "codemirror/theme/ttcn.css";
@import "codemirror/theme/twilight.css";
@import "codemirror/theme/vibrant-ink.css";
@import "codemirror/theme/xq-dark.css";
@import "codemirror/theme/xq-light.css";
@import "codemirror/theme/yeti.css";
@import "codemirror/theme/zenburn.css";
</style>
