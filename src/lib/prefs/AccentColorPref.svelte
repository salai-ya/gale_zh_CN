<script lang="ts">
	import Dropdown from '$lib/components/Dropdown.svelte';
	import Info from '$lib/components/Info.svelte';
	import Label from '$lib/components/Label.svelte';
	import { colors, getAccentColor, setAccentColor } from '$lib/theme';
	import { capitalize } from '$lib/util';

	let value = getAccentColor();
</script>

<div class="flex items-center">
	<Label>强调色</Label>

	<Info>更改高亮显示元素（如按钮和复选框）的颜色.</Info>

	<Dropdown
		class="grow"
		selected={value}
		items={colors}
		getLabel={capitalize}
		onSelectedChange={(newValue) => {
			value = newValue;
			setAccentColor(value);
		}}
		multiple={false}
	/>
</div>
