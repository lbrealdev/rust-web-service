document.getElementById('add-question-form').onsubmit = async (e) => {
  e.preventDefault();
  const title = document.getElementById('title').value;
  const content = document.getElementById('content').value;

  await fetch('/questions', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title, content }),
  });

  // back to home after create new question
  window.location.href = '/';
};

document.getElementById('back-btn').onclick = () => {
  window.location.href = '/';
};
