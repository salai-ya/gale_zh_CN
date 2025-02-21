<script lang="ts">
	import Label from '$lib/components/Label.svelte';
	import Dropdown from '$lib/components/Dropdown.svelte';
	import InputField from '$lib/components/InputField.svelte';

	import type { LaunchMode } from '$lib/models';
	import { sentenceCase } from '$lib/util';
	import { activeGame } from '$lib/stores';
	import Info from '$lib/components/Info.svelte';

	export let value: LaunchMode;
	export let set: (value: LaunchMode) => Promise<void>;

	let instances = value.content?.instances ?? 1;
	let intervalSecs = value.content?.intervalSecs ?? 10;

	async function onSelectedChange(newValue: string) {
		value.type = newValue as 'launcher' | 'direct';
		await submit();
	}

	async function submit() {
		if (value.type === 'direct') {
			value.content = { instances, intervalSecs };
		} else {
			value.content = undefined;
		}

		await set(value);
	}

	$: platforms = $activeGame?.platforms ?? [];
</script>

<div class="flex items-center">
	<Label>Launch mode</Label>

	<Info>
		<p>确定游戏的启动方式。</p>
		<p class="my-1.5">
			<b>Launcher:</b> 通过指定的平台启动。这对于某些游戏是必需的，对于
			例如，需要运行 Steam 才能工作。
		</p>
		<p>
			<b>引导:</b> 直接从可执行文件启动游戏。允许您启动多个实例
			立即。
		</p>
	</Info>

	<Dropdown
		class="grow"
		items={['launcher', 'direct']}
		getLabel={sentenceCase}
		selected={value?.type ?? 'direct'}
		multiple={false}
		disabled={platforms.length === 0}
		{onSelectedChange}
	/>
</div>

<div class="flex items-center">
	<Label>Number of instances</Label>

	<Info>How many instances of the game to launch at once. Only available in direct mode.</Info>

	<InputField
		disabled={value.type !== 'direct'}
		value={instances.toString()}
		on:change={({ detail }) => {
			instances = parseInt(detail);
			submit();
		}}
	/>
</div>

<div class="flex items-center">
	<Label>Interval between launches</Label>

	<Info>
		How many seconds to wait between launching each instance. Only applicable in direct mode with
		multiple instances.
	</Info>

	<InputField
		disabled={value.type !== 'direct' || instances <= 1}
		value={intervalSecs.toString()}
		on:change={({ detail }) => {
			intervalSecs = parseInt(detail);
			submit();
		}}
	/>
</div>
