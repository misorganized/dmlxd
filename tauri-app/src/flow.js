window.__TAURI__.tauri.invoke('is_first_login')
    .then(function(isFirstLogin) {
        if (isFirstLogin) {
            window.location.href = 'register.html';
        } else {
            window.location.href = 'main.html';
        }
    });
