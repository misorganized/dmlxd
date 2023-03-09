const registerButton = document.getElementById('register-button');
const invoke = window.__TAURI__.tauri.invoke;

registerButton.addEventListener('click', async () => {
    const username = document.getElementById('sign-up-username').value;
    const displayName = document.getElementById('sign-up-display-name').value;
    const port = document.getElementById('sign-up-port').value;
    const password = document.getElementById('sign-up-password').value;

    console.group('User registration');
    console.log('Username:', username);
    console.log('Display Name:', displayName);
    console.log('Port:', port);
    console.log('Password:', password)
    console.groupEnd();

    if (username === '' || displayName === '') {
        console.error('Username and display name cannot be empty');
        return;
    }

    const response = await invoke('init_register_user', {
        permanentLogin: username,
        loginPassword: password,
        displayName: displayName,
        portStr: port
    });
    console.log('Response:', response);
});
