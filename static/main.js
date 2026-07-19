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

function displayName(user) {
  if (!user) return '';
  return user.username || user.display_name || user.role || 'user';
}

function showLoginPrompt() {
  const loginModal = document.getElementById('login-modal');
  const loginForm = document.getElementById('login-form');
  const loginError = document.getElementById('login-error');
  const loginCancel = document.getElementById('login-cancel');
  const loginSubmitBtn = document.getElementById('login-submit-btn');
  const registerBtn = document.getElementById('register-btn');

  loginModal.classList.remove('hidden');
  loginError.textContent = '';
  loginError.classList.add('hidden');

  const finish = (data, message) => {
    setSession(data.token, data.user);
    if (data.sign_in_token) {
      localStorage.setItem('signInToken', data.sign_in_token);
      showAlert(
        `${message} Save your sign-in token (shown once in settings/storage): ${data.sign_in_token}`
      );
    } else {
      showAlert(message);
    }
    loginModal.classList.add('hidden');
    loginForm.reset();
    updateAuthUI();
    loadQuestions();
  };

  loginCancel.onclick = () => {
    loginModal.classList.add('hidden');
    loginError.textContent = '';
    loginError.classList.add('hidden');
    loginSubmitBtn.disabled = false;
    loginSubmitBtn.textContent = 'login';
  };

  loginForm.onsubmit = async (e) => {
    e.preventDefault();
    const username = loginForm.username.value.trim();
    const password = loginForm.password.value;
    const sign_in_token = loginForm.sign_in_token.value.trim();

    loginSubmitBtn.disabled = true;
    loginSubmitBtn.textContent = 'logging in…';
    loginError.classList.add('hidden');

    try {
      const body = sign_in_token
        ? { sign_in_token }
        : { username, password };
      const res = await fetch('/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body)
      });
      const data = await res.json().catch(() => ({}));
      if (res.ok && data.token) {
        finish(data, 'logged in successfully.');
      } else {
        loginError.textContent = data || 'login failed';
        if (typeof data === 'object' && data.message) {
          loginError.textContent = data.message;
        } else if (typeof data === 'string') {
          loginError.textContent = data;
        } else {
          loginError.textContent = 'login failed';
        }
        loginError.classList.remove('hidden');
      }
    } catch (err) {
      loginError.textContent = 'network error';
      loginError.classList.remove('hidden');
    } finally {
      loginSubmitBtn.disabled = false;
      loginSubmitBtn.textContent = 'login';
    }
  };

  registerBtn.onclick = async () => {
    const username = loginForm.username.value.trim();
    const password = loginForm.password.value;
    loginError.classList.add('hidden');
    registerBtn.disabled = true;
    try {
      const res = await fetch('/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password })
      });
      const text = await res.text();
      let data = {};
      try {
        data = JSON.parse(text);
      } catch {
        data = { message: text };
      }
      if (res.ok && data.token) {
        finish(data, 'account created.');
      } else {
        loginError.textContent = data.message || text || 'register failed';
        loginError.classList.remove('hidden');
      }
    } catch {
      loginError.textContent = 'network error';
      loginError.classList.remove('hidden');
    } finally {
      registerBtn.disabled = false;
    }
  };
}

async function continueWithGuestToken() {
  try {
    const res = await fetch('/auth/guest-token', { method: 'POST' });
    const data = await res.json();
    if (!res.ok || !data.token) {
      showAlert('could not create guest token.');
      return;
    }
    setSession(data.token, data.user);
    if (data.sign_in_token) {
      localStorage.setItem('signInToken', data.sign_in_token);
      showAlert(
        `guest session created. save your sign-in token to return later:\n${data.sign_in_token}`
      );
    }
    updateAuthUI();
    loadQuestions();
  } catch {
    showAlert('network error.');
  }
}

function updateAuthUI() {
  const loggedIn = isLoggedIn();
  const createBtn = document.getElementById('create-question-btn');
  const logoutBtn = document.getElementById('logout-btn');
  const loginBtn = document.getElementById('login-btn');
  const guestBtn = document.getElementById('guest-btn');
  const authLabel = document.getElementById('auth-label');

  if (loggedIn) {
    if (createBtn) createBtn.classList.remove('hidden');
    if (logoutBtn) logoutBtn.classList.remove('hidden');
    if (loginBtn) loginBtn.classList.add('hidden');
    if (guestBtn) guestBtn.classList.add('hidden');
    if (authLabel) {
      authLabel.textContent = displayName(getAuthUser());
      authLabel.classList.remove('hidden');
    }
  } else {
    if (createBtn) createBtn.classList.add('hidden');
    if (logoutBtn) logoutBtn.classList.add('hidden');
    if (loginBtn) loginBtn.classList.remove('hidden');
    if (guestBtn) guestBtn.classList.remove('hidden');
    if (authLabel) authLabel.classList.add('hidden');
  }
}

function loadQuestions() {
  fetch('/questions')
    .then((res) => res.json())
    .then((questions) => {
      const list = document.getElementById('questions-list');
      list.innerHTML = '';

      if (!questions || questions.length === 0) {
        const emptyLi = document.createElement('li');
        emptyLi.className = 'empty-state';
        emptyLi.textContent = 'no questions yet. be the first.';
        list.appendChild(emptyLi);
        return;
      }

      const me = getAuthUser();
      questions.forEach((q) => {
        const li = document.createElement('li');

        const info = document.createElement('div');
        info.className = 'question-info';

        const titleLink = document.createElement('a');
        titleLink.className = 'question-title';
        titleLink.href = `/question.html?id=${q.id}`;
        titleLink.textContent = `#${q.id} - ${q.title}`;
        info.appendChild(titleLink);

        if (q.tags && q.tags.length) {
          const tagsContainer = document.createElement('div');
          tagsContainer.className = 'tags-container';
          q.tags.forEach((tag) => {
            const tagSpan = document.createElement('span');
            tagSpan.className = 'tag-pill';
            tagSpan.textContent = tag;
            tagsContainer.appendChild(tagSpan);
          });
          info.appendChild(tagsContainer);
        }

        li.appendChild(info);

        const canDelete =
          isLoggedIn() &&
          me &&
          (me.role === 'admin' || me.id === q.author_id);

        if (canDelete) {
          const delBtn = document.createElement('button');
          delBtn.textContent = 'delete';
          delBtn.className = 'button button-delete';

          delBtn.onclick = async () => {
            const confirmDelete = await showConfirm(`delete question #${q.id}?`);
            if (confirmDelete) {
              apiFetch(`/questions/${q.id}`, { method: 'DELETE' })
                .then((res) => {
                  if (res.ok) {
                    showAlert(`question #${q.id} deleted.`);
                    loadQuestions();
                  } else if (res.status === 401) {
                    updateAuthUI();
                    showAlert('session expired — please log in again.');
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
    .catch((err) => {
      console.error('error loading questions:', err);
      showAlert('failed to load questions.');
    });
}

/* === Init =========================================================== */

document.addEventListener('DOMContentLoaded', () => {
  updateAuthUI();
  loadQuestions();

  const flash = sessionStorage.getItem('flash');
  if (flash) {
    sessionStorage.removeItem('flash');
    showAlert(flash);
  }

  const loginBtn = document.getElementById('login-btn');
  if (loginBtn) {
    loginBtn.addEventListener('click', () => {
      showLoginPrompt();
    });
  }

  const guestBtn = document.getElementById('guest-btn');
  if (guestBtn) {
    guestBtn.addEventListener('click', () => {
      continueWithGuestToken();
    });
  }

  const logoutBtn = document.getElementById('logout-btn');
  if (logoutBtn) {
    logoutBtn.addEventListener('click', async () => {
      await logoutRequest();
      updateAuthUI();
      loadQuestions();
      showAlert('logged out.');
    });
  }
});
