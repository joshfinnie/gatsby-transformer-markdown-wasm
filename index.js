const lib = require('./pkg/');

console.log(JSON.parse(lib.render("__strong__")));

const data = JSON.parse(lib.render("---\ntitle: Valid Yaml Test\n---\nsomething that's not yaml"));
console.log(data);
