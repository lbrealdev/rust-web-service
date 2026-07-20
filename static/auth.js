/* Shared auth helpers — session Bearer for mutating API calls */

function getSessionToken() {
  return localStorage.getItem('sessionToken');
}

function isLoggedIn() {
  return !!getSessionToken();
}

function getAuthUser() {
  try {
    return JSON.parse(localStorage.getItem('authUser') || 'null');
  } catch {
    return null;
  }
}

function setSession(token, user) {
  localStorage.setItem('sessionToken', token);
  localStorage.setItem('authUser', JSON.stringify(user || {}));
  localStorage.removeItem('loggedIn');
}

function clearSession() {
  localStorage.removeItem('sessionToken');
  localStorage.removeItem('authUser');
  localStorage.removeItem('loggedIn');
  localStorage.removeItem('signInToken');
}

function authHeaders(extra) {
  const headers = Object.assign({ 'Content-Type': 'application/json' }, extra || {});
  const token = getSessionToken();
  if (token) headers['Authorization'] = `Bearer ${token}`;
  return headers;
}

async function apiFetch(path, options) {
  const opts = options || {};
  const headers = authHeaders(opts.headers);
  const res = await fetch(path, Object.assign({}, opts, { headers }));
  if (res.status === 401 && opts.clearOn401 !== false) {
    clearSession();
  }
  return res;
}

async function logoutRequest() {
  try {
    await apiFetch('/logout', { method: 'POST', clearOn401: false });
  } catch {
    /* ignore */
  }
  clearSession();
}
