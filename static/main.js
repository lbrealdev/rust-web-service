function showAlert(message) {
  const alertModal = document.getElementById('alert');
  const alertMessage = document.getElementById('alert-message');
  const alertOk = document.getElementById('alert-ok');

  alertMessage.textContent = message;
  alertModal.classList.remove('hidden');

  alertOk.onclick = () => {
    alertModal.classList.add('hidden');
  };
}

function showConfirm(message) {
  return new Promise((resolve) => {
    const modal = document.getElementById('modal');
    const modalMessage = document.getElementById('modal-message');
    const modalConfirm = document.getElementById('modal-confirm');
    const modalCancel = document.getElementById('modal-cancel');

    modalMessage.textContent = message;
    modal.classList.remove('hidden');

    modalConfirm.onclick = () => {
      modal.classList.add('hidden');
      resolve(true);
    };

    modalCancel.onclick = () => {
      modal.classList.add('hidden');
      resolve(false);
    };
  });
}

function showLoginPrompt() {
  const loginModal = document.getElementById('login-modal');
  const loginForm = document.getElementById('login-form');
  const loginError = document.getElementById('login-error');
  const loginCancel = document.getElementById('login-cancel');

  loginModal.classList.remove('hidden');
  loginError.textContent = '';
  loginForm.password.focus();

  loginCancel.onclick = () => {
    loginModal.classList.add('hidden');
    loginError.textContent = '';
  };

  loginForm.onsubmit = async (e) => {
    e.preventDefault();
    const password = e.target.password.value;
    try {
      const res = await fetch('/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ password })
      });
      const data = await res.json();
      if (res.ok && data.success) {
        localStorage.setItem('loggedIn', 'true');
        loginModal.classList.add('hidden');
        loginError.textContent = '';
        e.target.password.value = '';
        updateAuthUI();
        loadQuestions();
        showAlert('Logged in successfully!');
      } else {
        loginError.textContent = data.message || 'Login failed';
        e.target.password.value = '';
        e.target.password.focus();
      }
    } catch (err) {
      loginError.textContent = 'Network error';
    }
  };
}

function updateAuthUI() {
  const loggedIn = localStorage.getItem('loggedIn') === 'true';
  const createBtn = document.getElementById('create-question-btn');
  const logoutBtn = document.getElementById('logout-btn');
  const loginBtn = document.getElementById('login-btn');

  if (loggedIn) {
    if (createBtn) createBtn.style.display = 'inline-block';
    if (logoutBtn) logoutBtn.style.display = 'inline-block';
    if (loginBtn) loginBtn.style.display = 'none';
  } else {
    if (createBtn) createBtn.style.display = 'none';
    if (logoutBtn) logoutBtn.style.display = 'none';
    if (loginBtn) loginBtn.style.display = 'inline-block';
  }
}

function loadQuestions() {
  fetch('/questions')
    .then(res => res.json())
    .then(questions => {
      const list = document.getElementById('questions-list');
      list.innerHTML = '';

      questions.forEach(q => {
        const li = document.createElement('li');

        const info = document.createElement('div');
        info.className = 'question-info';

        const titleSpan = document.createElement('span');
        titleSpan.className = 'question-title';
        titleSpan.textContent = `#${q.id} - ${q.title}`;
        info.appendChild(titleSpan);

        if (q.tags && q.tags.length) {
          const tagsContainer = document.createElement('div');
          tagsContainer.className = 'tags-container';
          q.tags.forEach(tag => {
            const tagSpan = document.createElement('span');
            tagSpan.className = 'tag-pill';
            tagSpan.textContent = tag;
            tagsContainer.appendChild(tagSpan);
          });
          info.appendChild(tagsContainer);
        }

        li.appendChild(info);

        if (localStorage.getItem('loggedIn') === 'true') {
          const delBtn = document.createElement('button');
          delBtn.textContent = 'Delete';
          delBtn.className = 'delete-button';

          delBtn.onclick = async () => {
            const confirmDelete = await showConfirm(`Delete question #${q.id}?`);
            if (confirmDelete) {
              fetch(`/questions/${q.id}`, { method: 'DELETE' })
                .then(res => {
                  if (res.ok) {
                    showAlert(`Question #${q.id} deleted.`);
                    loadQuestions();
                  } else {
                    showAlert(`Failed to delete question #${q.id}.`);
                  }
                })
                .catch(() => showAlert('Network error.'));
            }
          };

          li.appendChild(delBtn);
        }

        list.appendChild(li);
      });
    })
    .catch(err => {
      console.error('Error loading questions:', err);
      showAlert('Failed to load questions.');
    });
}

// Handle login/logout
document.addEventListener('DOMContentLoaded', () => {
  updateAuthUI();
  loadQuestions();

  const loginBtn = document.getElementById('login-btn');
  if (loginBtn) {
    loginBtn.addEventListener('click', () => {
      console.log('Login button clicked');
      showLoginPrompt();
    });
  }

  const logoutBtn = document.getElementById('logout-btn');
  if (logoutBtn) {
    logoutBtn.addEventListener('click', () => {
      localStorage.removeItem('loggedIn');
      updateAuthUI();
      loadQuestions();
      showAlert('Logged out.');
    });
  }
});
