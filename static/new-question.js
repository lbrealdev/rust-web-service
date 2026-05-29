document.getElementById('new-question-form').addEventListener('submit', async (e) => {
  e.preventDefault();

  const title = e.target.title.value.trim();
  const content = e.target.content.value.trim();
  const tagsRaw = e.target.tags.value.trim();
  const tags = tagsRaw ? tagsRaw.split(',').map(t => t.trim()) : null;

  try {
    const response = await fetch('/questions', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ title, content, tags })
    });

    if (!response.ok) {
      throw new Error('Failed to create question');
    }

    window.location.href = '/';
  } catch (err) {
    alert(err.message);
  }
});
