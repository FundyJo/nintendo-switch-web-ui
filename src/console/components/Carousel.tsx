import { EmblaCarouselType } from 'embla-carousel';
import useEmblaCarousel from 'embla-carousel-react';
import { WheelGesturesPlugin } from 'embla-carousel-wheel-gestures';
import { useEffect } from 'react';
import { useSnapshot } from 'valtio';

import state, { launchGame } from '../../state-tauri';

const removeExcessiveScroll = (emblaApi: EmblaCarouselType) => {
	emblaApi.on('scroll', (emblaApi) => {
		const {
			limit,
			target,
			location,
			scrollTo,
			translate,
			scrollBody,
		} = emblaApi.internalEngine();

		let edge: number | null = null;

		if (location.get() >= limit.max) edge = limit.max;
		if (location.get() <= limit.min) edge = limit.min;

		if (edge !== null) {
			location.set(edge);
			target.set(edge);
			translate.to(edge);
			translate.toggleActive(false);
			scrollBody.useDuration(0).useFriction(0);
			scrollTo.distance(0, false);
		} else {
			translate.toggleActive(true);
		}
	});
};

// TODO: On scroll start, remove selectedIndex
// TODO: Allow moving to edge first, before starting scrolling
const keyboardControl = (emblaApi: EmblaCarouselType) => {
	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.key === 'ArrowLeft' || event.key === 'ArrowRight') {
			const games = state.games;
			const maxIndex = Math.max(games.length - 1, 0);
			const currentIndex = state.selectedTitle !== null ? state.selectedTitle : emblaApi.selectedScrollSnap();
			const nextIndex = event.key === 'ArrowLeft'
				? Math.max(currentIndex - 1, 0)
				: Math.min(currentIndex + 1, maxIndex);

			if (nextIndex !== currentIndex) {
				state.selectedTitle = nextIndex;
				emblaApi.scrollTo(nextIndex);
			}
		} else if (event.key === 'Enter' || event.key === ' ') {
			// Launch game on Enter or Space
			if (state.selectedTitle !== null) {
				const games = state.games;
				if (state.selectedTitle < games.length) {
					const game = games[state.selectedTitle];
					if (game && game.path) {
						launchGame(game);
					}
				}
			}
		}
	};
	document.addEventListener('keydown', handleKeyDown);
	return () => document.removeEventListener('keydown', handleKeyDown);
};

export function Carousel() {
	const snap = useSnapshot(state);

	// Custom offset
	const customAlign = (viewSize: number, snapSize: number) => viewSize - snapSize;

	// Setup Embla
	const [emblaRef, emblaApi] = useEmblaCarousel({ loop: false, skipSnaps: true, align: customAlign, containScroll: 'keepSnaps' }, [WheelGesturesPlugin({ target: document.body, forceWheelAxis: 'y' })]);

	useEffect(() => {
		if (emblaApi) {
			console.log(emblaApi.slideNodes());
			removeExcessiveScroll(emblaApi);
			return keyboardControl(emblaApi);
		}
	}, [emblaApi]);

	const tileClicked = (index: number) => {
		state.selectedTitle = index;
	};

	// Get the current list of games (only scanned games)
	const games = snap.games;

	return (
		// <div className="mt-[-0.2em] h-[27em] overflow-hidden px-[10em]" ref={emblaRef}>
		<div className=" left-0 z-10 mt-[-0.2em] h-[27em] w-screen px-[10em] " ref={emblaRef}>
			<div className="flex size-full items-center gap-[1.3em]">
				{games.map((game, index) => {
					// Use the game's icon if available, otherwise use a placeholder
					const imageUrl = game.icon?.startsWith('data:') 
						? game.icon 
						: game.icon?.startsWith('http') 
						? game.icon 
						: game.icon 
						? `data:image/png;base64,${game.icon}`
						: 'https://via.placeholder.com/512x512/151515/FFFFFF?text=No+Icon';

					return (
						<div
							onClick={() => tileClicked(index)}
							className="relative aspect-square h-[24em] shrink-0 overflow-visible bg-[#151515]"
							key={game.id}
						>
							{/* Tile image */}
							<img src={imageUrl} alt={game.title} className="w-full h-full object-cover" />
							{/* Selected tile border */}
							{snap.selectedTitle === index && (
								<div className="animate-borderColor pointer-events-none absolute inset-[-.95em] rounded-[.2em] border-[.5em]"></div>
							)}
						</div>
					);
				})}
			</div>
		</div>
	);
}

export default Carousel;
