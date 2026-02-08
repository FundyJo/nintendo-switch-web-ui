import { useEffect } from 'react';

import Console from './console/Console';

function App() {
	const clicked = () => {
		const elem = document.documentElement;

		if (document.fullscreenElement) {
			if (document.exitFullscreen) {
				document.exitFullscreen();
			}
		} else {
			if (elem.requestFullscreen) {
				elem.requestFullscreen();
			}
		}
	};

	useEffect(() => {
		window.addEventListener('keydown', function (e) {
			if (['Space', 'ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].indexOf(e.code) > -1) {
				e.preventDefault();
			}
		}, false);

		window.addEventListener('scroll', () => {
			if (window.scrollX !== 0) {
				window.scrollTo(0, window.scrollY);
			}
		});
	}, []);

	return (
		<div className="h-screen w-screen" onDoubleClick={clicked}>
			<Console />
		</div>
	);
}

export default App;
