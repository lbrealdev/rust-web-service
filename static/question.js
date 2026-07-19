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

/* === Helpers ======================================================== */

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

function getQuestionId() {
  const params = new URLSearchParams(window.location.search);
  const id = params.get('id');
  return id ? Number(id) : NaN;
}

function renderQuestion(question) {
  document.title = question.title;
  document.getElementById('question-title').textContent = `#${question.id} — ${question.title}`;
  document.getElementById('question-content').textContent = question.content;

  const tagsEl = document.getElementById('question-tags');
  tagsEl.innerHTML = '';
  if (question.tags && question.tags.length) {
    question.tags.forEach((tag) => {
      const tagSpan = document.createElement('span');
      tagSpan.className = 'tag-pill';
      tagSpan.textContent = tag;
      tagsEl.appendChild(tagSpan);
    });
  }

  document.getElementById('question-detail').classList.remove('hidden');
  document.getElementById('answers-section').classList.remove('hidden');
}

function renderAnswers(answers) {
  const list = document.getElementById('answers-list');
  list.innerHTML = '';

  if (!answers || answers.length === 0) {
    const empty = document.createElement('li');
    empty.className = 'empty-state';
    empty.textContent = 'no answers yet. be the first.';
    list.appendChild(empty);
    return;
  }

  answers.forEach((answer) => {
    const li = document.createElement('li');
    li.className = 'answer-item';

    const meta = document.createElement('div');
    meta.className = 'answer-meta';
    meta.textContent = `#${answer.id}`;

    const content = document.createElement('p');
    content.className = 'answer-content';
    content.textContent = answer.content;

    li.appendChild(meta);
    li.appendChild(content);
    list.appendChild(li);
  });
}

async function loadQuestion(id) {
  const loading = document.getElementById('question-loading');
  const errorEl = document.getElementById('question-error');

  try {
    const [questionRes, answersRes] = await Promise.all([
      fetch(`/questions/${id}`),
      fetch(`/questions/${id}/answers`)
    ]);

    if (!questionRes.ok) {
      throw new Error('question not found');
    }

    const question = await questionRes.json();
    const answers = answersRes.ok ? await answersRes.json() : [];

    loading.classList.add('hidden');
    renderQuestion(question);
    renderAnswers(answers);
  } catch (err) {
    loading.classList.add('hidden');
    errorEl.textContent = err.message || 'failed to load question';
    errorEl.classList.remove('hidden');
  }
}

async function submitAnswer(id, content) {
  const res = await fetch('/answers', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ content, question_id: id })
  });

  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || 'failed to submit answer');
  }
}

/* === Init =========================================================== */

document.addEventListener('DOMContentLoaded', () => {
  const id = getQuestionId();
  if (!Number.isInteger(id) || id <= 0) {
    document.getElementById('question-loading').classList.add('hidden');
    const errorEl = document.getElementById('question-error');
    errorEl.textContent = 'missing or invalid question id';
    errorEl.classList.remove('hidden');
    return;
  }

  loadQuestion(id);

  const form = document.getElementById('add-answer-form');
  const submitBtn = document.getElementById('answer-submit-btn');
  const errorEl = document.getElementById('answer-error');

  form.addEventListener('submit', async (e) => {
    e.preventDefault();
    const content = form.content.value.trim();
    if (!content) return;

    submitBtn.disabled = true;
    submitBtn.textContent = 'submitting…';
    errorEl.textContent = '';
    errorEl.classList.add('hidden');

    try {
      await submitAnswer(id, content);
      form.reset();
      const answersRes = await fetch(`/questions/${id}/answers`);
      const answers = answersRes.ok ? await answersRes.json() : [];
      renderAnswers(answers);
      showAlert('answer added.');
    } catch (err) {
      errorEl.textContent = err.message;
      errorEl.classList.remove('hidden');
    } finally {
      submitBtn.disabled = false;
      submitBtn.textContent = 'submit answer';
    }
  });
});
