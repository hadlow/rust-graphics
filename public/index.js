const rust = import('./pkg/index.js');

rust.then(m => m.hello_world()).catch(console.error);
