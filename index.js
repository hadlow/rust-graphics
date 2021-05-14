const rust = import('./pkg/index_bg');
const canvas = document.getElementById('app');
const ctx = canvas.getContext('2d');

const MAX_FPS = 60.0;

rust.then(m => {
	const throttle = 1000.0 / MAX_FPS;
	const client = new m.Client();
	const startTime = Date.now();

	let lastDraw = -1;

	canvas.addEventListener('mousemove', (event) =>
	{
		client.update_mouse(event.offsetX, event.offsetY);
	});

	canvas.addEventListener('mousedown', (event) =>
	{
		client.update_mousedown(true);
	});

	canvas.addEventListener('mouseup', (event) =>
	{
		client.update_mousedown(false);
	});

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

			client.update(window.innerHeight, window.innerWidth);
			client.render(window.innerHeight, window.innerWidth);
		}
	}

	render();
});
