// const axios = require('axios');

// document.getElementById('add-password-form').addEventListener('submit', async (e) => {
//   e.preventDefault();
//   const username = document.getElementById('username').value;
//   const password = document.getElementById('password').value;

//   try {
//     await axios.post('http://127.0.0.1:3030/add_password', {
//       username,
//       encrypted_password: password,
//     });
//     alert('Password added successfully');
//   } catch (error) {
//     console.error(error);
//     alert('Failed to add password');
//   }
// });

// document.getElementById('get-passwords-form').addEventListener('submit', async (e) => {
//   e.preventDefault();
//   const masterPassword = document.getElementById('master-password').value;

//   try {
//     const response = await axios.get(`http://127.0.0.1:3030/passwords?master_password=${masterPassword}`);
//     const passwords = response.data;

//     const passwordList = document.getElementById('password-list');
//     passwordList.innerHTML = '';
//     passwords.forEach(password => {
//       const li = document.createElement('li');
//       li.textContent = `Username: ${password.username}, Password: ${password.encrypted_password}`;
//       passwordList.appendChild(li);
//     });
//   } catch (error) {
//     console.error(error);
//     alert('Failed to retrieve passwords');
//   }
// });



const axios = require('axios');

document.getElementById('add-password-form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const username = document.getElementById('username').value;
  const password = document.getElementById('password').value;

  try {
    const response = await axios.post('http://127.0.0.1:3030/add_password', {
      username,
      encrypted_password: password,
    });
    alert('Password added successfully');
    console.log('Success:', response.data);
  } catch (error) {
    console.error('Error adding password:', error.response ? error.response.data : error.message);
    alert('Failed to add password');
  }
});

document.getElementById('get-passwords-form').addEventListener('submit', async (e) => {
  e.preventDefault();
  const masterPassword = document.getElementById('master-password').value;

  try {
    const response = await axios.get(`http://127.0.0.1:3030/passwords?master_password=${masterPassword}`);
    const passwords = response.data;

    const passwordList = document.getElementById('password-list');
    passwordList.innerHTML = '';
    passwords.forEach(password => {
      const li = document.createElement('li');
      li.textContent = `Username: ${password.username}, Password: ${password.encrypted_password}`;
      passwordList.appendChild(li);
    });
    console.log('Passwords retrieved:', passwords);
  } catch (error) {
    console.error('Error retrieving passwords:', error.response ? error.response.data : error.message);
    alert('Failed to retrieve passwords');
  }
});

document.getElementById('generate-password').addEventListener('click', () => {
  const generatedPassword = generateRandomPassword(50);
  document.getElementById('generated-password').value = generatedPassword;
  document.getElementById('password').value = generatedPassword;
});

function generateRandomPassword(length) {
  const charset = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+~`|}{[]:;?><,./-=';
  let password = '';
  for (let i = 0, n = charset.length; i < length; ++i) {
    password += charset.charAt(Math.floor(Math.random() * n));
  }
  return password;
}
