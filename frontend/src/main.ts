import App from './App.svelte'
import '@material/mwc-button'
import '@material/mwc-tab-bar'
import '@material/mwc-tab'
import '@material/mwc-textfield'
import '@material/mwc-top-app-bar'
import '@material/mwc-icon-button'
import '@material/mwc-list'
import '@material/mwc-dialog'
import '@material/mwc-formfield'
import '@material/mwc-radio'
import '@material/mwc-fab'

import {Chart, PieController} from "chart.js";
Chart.register(PieController)

const app = new App({
  target: document.querySelector('body')
})

export default app
