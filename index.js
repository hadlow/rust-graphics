const rust = import('./pkg/index_bg');

rust.then(m => m.hello_world()).catch(console.error);
