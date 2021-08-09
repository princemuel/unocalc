//This function inspects overflowing elements 
const docWidth = document.documentElement.offsetWidth;
[].forEach.call(document.querySelectorAll('*'), (el) =>
  console.log(el.offsetWidth > docWidth)
);
/*////*/