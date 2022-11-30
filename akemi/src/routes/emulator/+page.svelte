<script lang="ts">
	import '@styles/page_emulator.sass';
	import { browser } from '$app/environment';
	import * as wasm from 'tomo';

	let emulator: wasm.Cpu;
	let running = false;
	let audio;

	if (browser) {
		emulator = new wasm.Cpu();
	}

	// Laden der ROM
	async function handleChange(e) {
		const file: File = e.target.files[0];
		const text = await file.arrayBuffer();
		const bytes = new Uint8Array(text);
		console.log(bytes);
		emulator.load_rom(bytes);
		alert('ROM loaded');
	}


	function startStop() {
		if (running) {
			running = false;
			emulator.reset();
			updateDisplay(Uint32Array.from(Array(64*32)))
		} else {
			running = true;
		}
	}

	function gameLoop() {
		console.log('loop');
		let output = emulator.tick();
		if (!output.success) {
			running = false;
			alert("ERROR: The emulator was unable to deconstruct and execute the provided opcode")
		}

		if (emulator.should_beep() && audio) {
			audio.play();
		}

		// Display neu zeichnen
		updateDisplay(output.edited_pixels);

		// Um die 60hz stabil zu halten
		if (running) window.requestAnimationFrame(gameLoop);
	}

	$: if (running) gameLoop();

	function updateDisplay(updates: Uint32Array) {
		console.log("update Display called");

		for (let i = 0; i < updates.length; i += 1) {
			console.log("update pixels");
			let j = updates[i]
			let ref = refs[j];
			if (!ref) {
				console.log("No ref! I: ", i);
			}
			if (ref.classList.contains("on") && !emulator.display.get_pixel_state(j)) {
				ref.classList.remove("on");
				ref.classList.add("off");
			} else if (ref.classList.contains("off") && emulator.display.get_pixel_state(j)) {
				ref.classList.remove("off");
				ref.classList.add("on");
			}
		}
	}

	let refs: Array<HTMLElement> = Array(64 * 32);

	function getPixelData(position: number): boolean {
		if (!browser) return false;
		return emulator.display.get_pixel_state(position);

	}
</script>

<main>
	<audio src="https://www.soundjay.com/buttons/beep-02.mp3" bind:this={audio}>
	</audio>
	<div class="display">
		{#each Array(64 * 32) as _, i}
			<div class:on={getPixelData(i)} class:off={!getPixelData(i)} bind:this={refs[i]}>
			</div>
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
