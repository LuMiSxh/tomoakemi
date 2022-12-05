<script lang="ts">
	let width: number;
	let arr;
	$: arr = Math.abs(Math.ceil(width / 20));
</script>

<svelte:window bind:innerWidth={width} />
<main class="component bubble-background">
	<div class="component bubble-content">
		<slot />
	</div>
	<div class="component bubble-particles">
		{#each Array(arr ? arr : 64) as _, i}
			<div id={i} />
		{/each}
	</div>
</main>
