const rust = import('./pkg/index_bg');
const canvas = document.getElementById('app');
const gl = canvas.getContext('webgl', {
	antialias: true,
});

const MAX_FPS = 60.0;

rust.then(m => {
	if(!gl) {
		console.log('Error initializing GL');
		return;
	}

	gl.enable(gl.BLEND);
	gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

	const throttle = 1000.0 / MAX_FPS;
	const client = new m.Client();
	const startTime = Date.now();

	let lastDraw = -1;

	function render() {
		window.requestAnimationFrame(render);

		const time = Date.now();

		if(time >= lastDraw + throttle) {
			lastDraw = time;

			if(window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
				canvas.height = window.innerHeight;
				canvas.clientHeight = window.innerHeight;
				canvas.style.height = window.innerHeight;

				canvas.width = window.innerWidth;
				canvas.clientWidth = window.innerWidth;
				canvas.style.width = window.innerWidth;

				gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

			let elapsedTime = time - startTime;

			client.update(elapsedTime, window.innerHeight, window.innerWidth);
			client.render();
		}
	}

	render();
});
