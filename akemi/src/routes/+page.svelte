<script lang="ts">
	import Tooltip from '../components/Tooltip.svelte';
	import Waves from '@components/Bubbles.svelte';

	import { base } from '$app/paths';
</script>

<Waves>
	<main class="grid-container">
		<div class="grid-content center-all">
			<h1>
				Welcome to
				<Tooltip title="Tomo-Akemi"><span class="project-name">ともあけみ</span></Tooltip>
			</h1>
		</div>
		<div class="grid-content center-all">
			<a href="{base}/emulators" class="landing-link">Get started</a>
		</div>
	</main>
</Waves>
