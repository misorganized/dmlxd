const { invoke } = window.__TAURI__.tauri;
const { invoke } = require('@tauri-apps/api/tauri')

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#greet-button")
    .addEventListener("click", () => greet());
});

// Define an endpoint that calls the `register_user` function in your Rust backend
async function registerUser(permanentLogin, displayName, password) {
  const result = await invoke('register_user', permanentLogin, displayName, password)
  return result
}