<script lang="ts">
	import '@styles/page_emulator.sass';
	import { browser } from '$app/environment';
	import * as wasm from 'tomo';

	let emulator: wasm.Cpu;
	let bug_fix = 'If you can see me, ignore me or refresh the page :3';
	let running = false;
	let display_loop = Array(64 * 32);
	let refs: Array<HTMLElement> = Array(64 * 32);
	let audio;

	if (browser) {
		emulator = new wasm.Cpu();
		bug_fix = '';
	}

	// Laden der ROM
	async function handleChange(e) {
		const file: File = e.target.files[0];
		const text = await file.text();
		const bytes = new Uint8Array(file.size);
		for (let i = 0; i < text.length; i++) {
			bytes[i] = text.charCodeAt(i);
		}
		console.log(bytes);
		emulator.load_rom(bytes);
		alert('ROM loaded');
	}

	// Selbsterklärend
	function getPixelData(position: number): boolean {
		if (!browser) return false;
		return emulator.display.get_pixel_single(position);
	}

	function startStop() {
		if (running) {
			running = false;
			emulator.reset();
		} else {
			running = true;
		}
	}

	function gameLoop() {
		console.log('loop');
		let success = emulator.tick();
		if (!success) {
			running = false;
			alert("ERROR: The emulator was unable to deconstruct and execute the provided opcode")
		}
		// TODO: Geht nicht
		// Brauchen wir um den Bildschirm zeu zu zeichnen
		for (let i = 0; i < 64*32; i++) {
			console.log('update pixels');
			let ref = refs[i];
			if (!ref) {
				console.log("No ref! I: ", i)
			}
			if (ref.classList.contains('on') && !emulator.display.get_pixel_single(i)) {
				ref.classList.remove('on');
				ref.classList.add('off');
			} else if (ref.classList.contains('off') && emulator.display.get_pixel_single(i)) {
				ref.classList.remove('off');
				ref.classList.add('on');
			}
		}

		if (emulator.should_beep() && audio) {
			audio.play();
		}

		// Um die 60hz stabil zu halten
		if (running) window.requestAnimationFrame(gameLoop);
	}

	$: if (running) gameLoop();
</script>

<main>
	<audio src="https://www.soundjay.com/buttons/beep-02.mp3" bind:this={audio} />
	<div class="emulator_window">
		{bug_fix}
		{#each display_loop as _, i}
			<div class:on={getPixelData(i)} class:off={!getPixelData(i)} bind:this={refs[i]} />
		{/each}
	</div>
	<div class="settings">
		<h1 class="fancy">Settings</h1>
		<label for="rom">
			Upload a ROM to play:
			<input on:change={handleChange} id="rom" name="ROM" type="file" />
		</label>
		<button on:click={startStop}>{running ? 'Reset' : 'Start'}</button>
	</div>
</main>
