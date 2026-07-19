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

/* === Auth gate (#65) ================================================ */

if (localStorage.getItem('loggedIn') !== 'true') {
  window.alert('login required to create a question.');
  window.location.replace('/');
}

/* === Create form ==================================================== */

document.addEventListener('DOMContentLoaded', () => {
  if (localStorage.getItem('loggedIn') !== 'true') {
    return;
  }

  const form = document.getElementById('new-question-form');
  if (!form) return;

  form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const title = form.title.value.trim();
    const content = form.content.value.trim();
    const tagsRaw = form.tags.value.trim();
    const tags = tagsRaw ? tagsRaw.split(',').map((t) => t.trim()).filter(Boolean) : null;
    const submitBtn = document.getElementById('submit-btn');
    const errorEl = document.getElementById('form-error');

    submitBtn.disabled = true;
    submitBtn.textContent = 'creating…';
    errorEl.textContent = '';
    errorEl.classList.add('hidden');

    try {
      const response = await fetch('/questions', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ title, content, tags })
      });

      if (!response.ok) {
        const data = await response.json().catch(() => ({}));
        throw new Error(data.message || 'failed to create question');
      }

      window.location.href = '/';
    } catch (err) {
      errorEl.textContent = err.message;
      errorEl.classList.remove('hidden');
    } finally {
      submitBtn.disabled = false;
      submitBtn.textContent = 'submit question';
    }
  });
});
