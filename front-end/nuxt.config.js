const path = require("path");

const { NormalModuleReplacementPlugin } = require("webpack");

require("dotenv").config({ path: path.resolve(process.cwd(), "../.env") });

module.exports = {
  env: {
    API_URL: "http://wall.ocboogie.com/api",
    ...process.env
  },
  /*
  ** Headers of the page
  */
  head: {
    title: "wall-front-end",
    meta: [
      { charset: "utf-8" },
      { name: "viewport", content: "width=device-width, initial-scale=1" },
      { hid: "description", name: "description", content: "Nuxt.js project" }
    ],
    link: [{ rel: "icon", type: "image/x-icon", href: "/favicon.ico" }]
  },
  /*
  ** Customize the progress bar color
  */
  loading: { color: "#3B8070" },
  /*
  ** Build configuration
  */
  plugins: ["~/plugins/element-ui"],
  build: {
    vendor: ["codemirror"],
    /*
    ** Run ESLint on save
    */
    extend(config, { isDev, isClient }) {
      if (isDev && isClient) {
        config.module.rules.push({
          enforce: "pre",
          test: /\.(js|vue)$/,
          loader: "eslint-loader",
          exclude: /(node_modules)/
        });
      }
      config.plugins.push(
        new NormalModuleReplacementPlugin(
          // eslint-disable-next-line no-useless-escape
          /element-ui[\/\\]lib[\/\\]locale[\/\\]lang[\/\\]zh-CN/,
          "element-ui/lib/locale/lang/en"
        )
      );
    }
  }
};
