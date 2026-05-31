document.getElementById('new-question-form').addEventListener('submit', async (e) => {
  e.preventDefault();

  const form = e.target;
  const title = form.title.value.trim();
  const content = form.content.value.trim();
  const tagsRaw = form.tags.value.trim();
  const tags = tagsRaw ? tagsRaw.split(',').map(t => t.trim()) : null;
  const submitBtn = document.getElementById('submit-btn');
  const errorEl = document.getElementById('form-error');

  submitBtn.disabled = true;
  submitBtn.textContent = 'Creating…';
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
      throw new Error(data.message || 'Failed to create question');
    }

    window.location.href = '/';
  } catch (err) {
    errorEl.textContent = err.message;
    errorEl.classList.remove('hidden');
  } finally {
    submitBtn.disabled = false;
    submitBtn.textContent = 'Submit Question';
  }
});
