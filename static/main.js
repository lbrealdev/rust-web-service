/* === Theme Toggle =================================================== */

(function () {
  const savedTheme = localStorage.getItem('theme');
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;

  if (savedTheme) {
    document.documentElement.setAttribute('data-theme', savedTheme);
  } else if (prefersDark) {
    document.documentElement.setAttribute('data-theme', 'dark');
  }

  document.addEventListener('DOMContentLoaded', () => {
    const toggle = document.getElementById('theme-toggle');
    if (!toggle) return;

    toggle.addEventListener('click', () => {
      const current = document.documentElement.getAttribute('data-theme') || 'light';
      const next = current === 'light' ? 'dark' : 'light';
      document.documentElement.setAttribute('data-theme', next);
      localStorage.setItem('theme', next);
    });
  });
})();

/* === UI Helpers ===================================================== */

function showAlert(message) {
  const alertMessage = document.getElementById('alert-message');
  const alertModal = document.getElementById('alert');
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
  const loginSubmitBtn = document.getElementById('login-submit-btn');
  const passwordInput = loginForm.password;

  loginModal.classList.remove('hidden');
  loginError.textContent = '';
  loginError.classList.add('hidden');
  passwordInput.focus();

  loginCancel.onclick = () => {
    loginModal.classList.add('hidden');
    loginError.textContent = '';
    loginError.classList.add('hidden');
    loginSubmitBtn.disabled = false;
    loginSubmitBtn.textContent = 'login';
  };

  loginForm.onsubmit = async (e) => {
    e.preventDefault();
    const password = passwordInput.value;

    loginSubmitBtn.disabled = true;
    loginSubmitBtn.textContent = 'logging in…';

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
        loginError.classList.add('hidden');
        passwordInput.value = '';
        updateAuthUI();
        loadQuestions();
        showAlert('logged in successfully.');
      } else {
        loginError.textContent = data.message || 'login failed';
        loginError.classList.remove('hidden');
        passwordInput.value = '';
        passwordInput.focus();
      }
  } catch (err) {
    loginError.textContent = 'network error';
    loginError.classList.remove('hidden');
  } finally {
      loginSubmitBtn.disabled = false;
      loginSubmitBtn.textContent = 'login';
    }
  };
}

function updateAuthUI() {
  const loggedIn = localStorage.getItem('loggedIn') === 'true';
  const createBtn = document.getElementById('create-question-btn');
  const logoutBtn = document.getElementById('logout-btn');
  const loginBtn = document.getElementById('login-btn');

  if (loggedIn) {
    if (createBtn) createBtn.classList.remove('hidden');
    if (logoutBtn) logoutBtn.classList.remove('hidden');
    if (loginBtn) loginBtn.classList.add('hidden');
  } else {
    if (createBtn) createBtn.classList.add('hidden');
    if (logoutBtn) logoutBtn.classList.add('hidden');
    if (loginBtn) loginBtn.classList.remove('hidden');
  }
}

function loadQuestions() {
  fetch('/questions')
    .then(res => res.json())
    .then(questions => {
      const list = document.getElementById('questions-list');
      list.innerHTML = '';

      if (!questions || questions.length === 0) {
        const emptyLi = document.createElement('li');
        emptyLi.className = 'empty-state';
        emptyLi.textContent = 'no questions yet. be the first.';
        list.appendChild(emptyLi);
        return;
      }

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
          delBtn.textContent = 'delete';
          delBtn.className = 'button button-delete';

          delBtn.onclick = async () => {
            const confirmDelete = await showConfirm(`delete question #${q.id}?`);
            if (confirmDelete) {
              fetch(`/questions/${q.id}`, { method: 'DELETE' })
                .then(res => {
                  if (res.ok) {
                    showAlert(`question #${q.id} deleted.`);
                    loadQuestions();
                  } else {
                    showAlert(`failed to delete question #${q.id}.`);
                  }
                })
                .catch(() => showAlert('network error.'));
            }
          };

          li.appendChild(delBtn);
        }

        list.appendChild(li);
      });
    })
    .catch(err => {
      console.error('error loading questions:', err);
      showAlert('failed to load questions.');
    });
}

/* === Init =========================================================== */

document.addEventListener('DOMContentLoaded', () => {
  updateAuthUI();
  loadQuestions();

  const loginBtn = document.getElementById('login-btn');
  if (loginBtn) {
    loginBtn.addEventListener('click', () => {
      showLoginPrompt();
    });
  }

  const logoutBtn = document.getElementById('logout-btn');
  if (logoutBtn) {
    logoutBtn.addEventListener('click', () => {
      localStorage.removeItem('loggedIn');
      updateAuthUI();
      loadQuestions();
      showAlert('logged out.');
    });
  }
});
