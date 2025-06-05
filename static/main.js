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

function loadQuestions() {
  fetch('/questions')
    .then(res => res.json())
    .then(questions => {
      const list = document.getElementById('questions-list');
      list.innerHTML = '';

      questions.forEach(q => {
        const li = document.createElement('li');

        const span = document.createElement('span');
        span.textContent = `#${q.id} - ${q.title}`;
        li.appendChild(span);

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
        list.appendChild(li);
      });
    })
    .catch(err => {
      console.error('Error loading questions:', err);
      showAlert('Failed to load questions.');
    });
}

loadQuestions();
