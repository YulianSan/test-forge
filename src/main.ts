import { createApp } from "vue";
import App from "./App.vue";
import './style.css'
import { OhVueIcon, addIcons } from "oh-vue-icons";
import { FaFlag } from "oh-vue-icons/icons";

addIcons(FaFlag);

createApp(App)
    .component('v-icon', OhVueIcon)
    .mount("#app");
