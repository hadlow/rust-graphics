const rust = import('./pkg/index_bg');
const canvas = document.getElementById('app');
const ctx = canvas.getContext('2d');

const MAX_FPS = 60.0;

rust.then(m => {
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
            }

			let elapsedTime = time - startTime;

			client.update(elapsedTime, window.innerHeight, window.innerWidth);
			client.render();
		}
	}

	render();
});
