function loadQuestions() {
  fetch('/questions')
    .then(res => res.json())
    .then(questions => {
      const list = document.getElementById('questions-list');
      list.innerHTML = '';  // limpa lista antes de carregar

      questions.forEach(q => {
        const li = document.createElement('li');
        li.textContent = `#${q.id} - ${q.title}: ${q.content} `;

        // Botão deletar
        const delBtn = document.createElement('button');
        delBtn.textContent = 'Delete';
        delBtn.className = 'button delete-button';
        delBtn.style.marginLeft = '15px';

        delBtn.onclick = () => {
          if (confirm(`Delete question #${q.id}?`)) {
            fetch(`/questions/${q.id}`, { method: 'DELETE' })
              .then(res => {
                if (res.ok) {
                  alert(`Question #${q.id} deleted.`);
                  loadQuestions(); // recarrega lista
                } else {
                  alert(`Failed to delete question #${q.id}.`);
                }
              })
              .catch(() => alert('Network error.'));
          }
        };

        li.appendChild(delBtn);
        list.appendChild(li);
      });
    })
    .catch(err => console.error('Error loading questions:', err));
}

// Carrega perguntas ao abrir a página
loadQuestions();
