<template>
  <el-card class="box-card">
    <div slot="header" class="clearfix">
      <span>Settings</span>
    </div>
    <el-row>
      <el-col class="label" :span="8">
        Theme:
      </el-col>
      <el-col :span="16">
        <el-select v-model="theme" @change="$emit('theme-change', $event)" filterable>
          <el-option
            v-for="theme in themes"
            :key="theme.value"
            :label="theme.label"
            :value="theme.value">
          </el-option>
        </el-select>
      </el-col>
    </el-row>
    <el-row>
      <el-col class="label" :span="8">
        Language:
      </el-col>
      <el-col :span="16">
        <el-select v-model="theme" @change="$emit('theme-change', $event)" filterable>
          <el-option
            v-for="theme in themes"
            :key="theme.value"
            :label="theme.label"
            :value="theme.value">
          </el-option>
        </el-select>
      </el-col>
    </el-row>
    <!-- <el-select class="dropdown" :value="theme" @change="$emit('theme-change', $event)" placeholder="Theme" filterable default-first-option>
      <el-option
        v-for="item in themes"
        :key="item.value"
        :value="item.value"
        :label="item.label"/>
    </el-select> -->
    <!-- <br/>
    Language: -->
    <!-- <el-select class="dropdown" placeholder="Language" :options="languages" :value="['plain']" @change="$emit('language-change', $event)" filterable/> -->
  </el-card>
</template>
<script>
import startCase from "lodash.startcase";
import themesJson from "~/assets/themes.json";

// let CodeMirror;
if (process.browser) {
  CodeMirror = require("codemirror");
  require("codemirror/mode/meta");
  console.log(CodeMirror.modeInfo);
  // require("codemirror/mode/javascript/javascript.js");
}

themesJson.unshift("default");

const themes = themesJson.map(value => ({
  value,
  label: startCase(value)
}));

export default {
  data() {
    // if (process.browser) {
    //   return {
    //     theme: "",
    //     themes,
    //     languages: Object.values(CodeMirror.modeInfo).map(language => ({
    //       label: language.name,
    //       value: language.mode
    //     }))
    //   };
    // }
    return {
      theme: "default",
      themes,
      languages: []
    };
  }
};
</script>
<style lang="scss" scoped>
.box-card {
  width: 280px;
  .label {
    padding-top: 12px;
  }
  .field {
    .dropdown {
      // margin: auto;
      // width: 50%;
    }
  }
}
</style>


