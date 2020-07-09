const lib = require('./pkg/');

console.log(lib.render("__strong__"));

const data = lib.render("---\ntitle: Valid Yaml Test\n---\nsomething that's not yaml");

console.log(data.data);

const bigMarkdown = `---

title: "First Post"
date: "2010-01-25"
tags:
  - "announcement"
  - "news"
path: "/blog/first-post"
expires: true

---

Hello and welcome to Josh Finnie.com. I am very excited that I got Jekyll up and running on my new server at [Webfaction](http://webfaction.com/). I appologize in advance for the look and feel of this website as I am trying to get the technology down before I work on its looks.`;

console.time()
lib.render(bigMarkdown);
console.timeEnd();
