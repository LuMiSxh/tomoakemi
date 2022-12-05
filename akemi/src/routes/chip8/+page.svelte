<script lang="ts">
	import { browser } from '$app/environment';
	import * as chip8 from 'tomo';
	import { onMount } from 'svelte';
	// CUP
	let emu: chip8.Processor;
	let running = false;
	if (browser) {
		emu = new chip8.Processor();
	}

	// Display
	let innerWidth: number;
	let scale = 30;
	$: scale = Math.abs(Math.ceil(innerWidth / 70));
	if (scale % 2 !== 0) {
		scale += 1;
	}
	let canv: HTMLCanvasElement;
	let colorOn = 'white';
	let colorOff = 'black';

	function cls() {
		const ctx = canv.getContext('2d');
		ctx.beginPath();
		ctx.rect(0, 0, 64 * scale, 32 * scale);
		ctx.fillStyle = colorOff;
		ctx.fill();
	}

	onMount(() => {
		setTimeout(() => {
			cls();
		}, 100);
	});

	// Audio and Misc
	let audio;

	// Handle loading a rom
	async function loadData(e) {
		const file: File = e.target.files[0];
		const text = await file.arrayBuffer();
		const bytes = new Uint8Array(text);
		let length = emu.load(bytes);
		alert('ROM was loaded successfully. \nDEBUG: Loaded data has a length of ' + length + ' bytes');
	}

	// Handle starting and stopping the emulator
	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	function startStop() {
		if (running) {
			running = false;
			emu.reset();
			cls();
		} else {
			running = true;
		}
	}

	// Main game loop
	$: if (running) gameLoop();

	function gameLoop() {
		if (!running) return;

		let output = emu.tick();

		if (!output.success) {
			running = false;
			alert(
				"Something went wrong processing the latest opcode. \nDEBUG: The faulty opcode is '" +
					output.opcode.toString(16) +
					"'"
			);
		}

		if (emu.should_beep()) {
			audio.play();
		}

		const ctx = canv.getContext('2d');

		for (let y = 0; y < 32; y++) {
			for (let x = 0; x < 64; x++) {
				ctx.beginPath();
				ctx.rect(x * scale, y * scale, x + scale, y + scale);

				if (emu.display.get_pixel(y, x)) {
					ctx.fillStyle = colorOn;
				} else {
					ctx.fillStyle = colorOff;
				}
				ctx.fill();
			}
		}

		if (running) window.requestAnimationFrame(gameLoop);
	}

	// Keyboard handler
	function keyDown(e) {
		switch (e.keyCode) {
			// 0
			case 48:
				emu.key_press(9);
				break;
			// 1
			case 49:
				emu.key_press(0);
				break;
			// 2
			case 50:
				emu.key_press(1);
				break;
			// 3
			case 51:
				emu.key_press(2);
				break;
			// 4
			case 52:
				emu.key_press(3);
				break;
			// 5
			case 53:
				emu.key_press(4);
				break;
			// 6
			case 54:
				emu.key_press(5);
				break;
			// 7
			case 55:
				emu.key_press(6);
				break;
			// 8
			case 56:
				emu.key_press(7);
				break;
			// 9
			case 57:
				emu.key_press(8);
				break;
			// 1
			case 65:
				emu.key_press(10);
				break;
			// 2
			case 66:
				emu.key_press(11);
				break;
			// 3
			case 67:
				emu.key_press(12);
				break;
			// 4
			case 68:
				emu.key_press(13);
				break;
			// 5
			case 69:
				emu.key_press(14);
				break;
			// 6
			case 70:
				emu.key_press(15);
				break;
		}
	}

	function keyUp(e) {
		switch (e.keyCode) {
			// 0
			case 48:
				emu.key_up(9);
				break;
			// 1
			case 49:
				emu.key_up(0);
				break;
			// 2
			case 50:
				emu.key_up(1);
				break;
			// 3
			case 51:
				emu.key_up(2);
				break;
			// 4
			case 52:
				emu.key_up(3);
				break;
			// 5
			case 53:
				emu.key_up(4);
				break;
			// 6
			case 54:
				emu.key_up(5);
				break;
			// 7
			case 55:
				emu.key_up(6);
				break;
			// 8
			case 56:
				emu.key_up(7);
				break;
			// 9
			case 57:
				emu.key_up(8);
				break;
			// 1
			case 65:
				emu.key_up(10);
				break;
			// 2
			case 66:
				emu.key_up(11);
				break;
			// 3
			case 67:
				emu.key_up(12);
				break;
			// 4
			case 68:
				emu.key_up(13);
				break;
			// 5
			case 69:
				emu.key_up(14);
				break;
			// 6
			case 70:
				emu.key_up(15);
				break;
		}
	}
</script>

<svelte:window bind:innerWidth on:keydown={keyDown} on:keyup={keyUp} />
<main class="center-all">
	<audio src="https://www.soundjay.com/buttons/beep-02.mp3" bind:this={audio} />
	<div class="center-all">
		<canvas bind:this={canv} width={64 * scale} height={32 * scale} />
	</div>
	<div class="settings-wrapper center-all">
		<h1>Settings</h1>
		<div class="height-limited-grid-container">
			<div class="height-limited-grid-container center-all">
				<label for="rom">
					Upload a ROM to play:
					<input on:change={loadData} id="rom" name="ROM" type="file" />
				</label>
				<label for="colorON">
					{running ? 'Resetting' : 'Starting'} the emulator
					<button
						class:starter={!running}
						class:stopper={running}
						class="btn-wee"
						id="wee"
						on:click={startStop}
					/>
				</label>
			</div>
			<div class="height-limited-grid-container center-all">
				<label for="colorON">
					Set a color for active pixels
					<input bind:value={colorOn} id="colorON" on:keydown />
				</label>
				<label for="colorOFF">
					Set a color for inactive pixels
					<input bind:value={colorOff} id="colorOFF" on:keydown />
				</label>
			</div>
		</div>
	</div>
</main>
