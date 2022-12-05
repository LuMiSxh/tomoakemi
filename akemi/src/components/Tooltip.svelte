<script lang="ts">
	export let title = '';
	let isHovered = false;
	let x;
	let y;

	function mouseOver(event) {
		isHovered = true;
		x = event.pageX + 5;
		y = event.pageY + 5;
	}
	function mouseMove(event) {
		x = event.pageX + 5;
		y = event.pageY + 5;
	}
	function mouseLeave() {
		isHovered = false;
	}
</script>

<div
	class="component tooltip-wrapper"
	on:focus={{}}
	on:mouseover={mouseOver}
	on:mouseleave={mouseLeave}
	on:mousemove={mouseMove}
>
	<slot />
</div>

{#if isHovered}
	<div style="top: {y}px; left: {x}px;" class="component tooltip">{title}</div>
{/if}
