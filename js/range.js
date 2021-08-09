const _R = document.querySelector('[type=range]');

document.documentElement.classList.add('js');

_R.addEventListener('input', e => {
  _R.style.setProperty('--val', +_R.value)
}, false);