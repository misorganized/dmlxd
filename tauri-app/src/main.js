const { invoke } = window.__TAURI__.tauri;
//const { invoke } = require('@tauri-apps/api/tauri')


const title = document.querySelector('.title');
const chars = title.textContent.split('');
title.innerHTML = chars.map(c => `<span>${c}</span>`).join('');
